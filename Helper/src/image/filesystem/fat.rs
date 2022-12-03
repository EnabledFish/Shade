use std::io::Cursor;
use std::ops::Range;
use std::path::PathBuf;

use fatfs::{Dir, FileSystem, FormatVolumeOptions, FsOptions, StdIoWrapper, Write};
use mbrman::{BOOT_INACTIVE, CHS, MBR, MBRPartitionEntry};

pub fn create_image_fat(
    from_folder: &PathBuf,
    to_file: &PathBuf,
    sector_size: usize,
    sector_count: usize,
) {
    let partition_byte_range;
    let mut disk_image = vec![0; sector_size * sector_count];

    {
        let mut cursor = Cursor::new(&mut disk_image);
        let mut mbr = MBR::new_from(
            &mut cursor,
            sector_size as u32,
            [0xff; 4],
        ).unwrap();

        mbr[1] = MBRPartitionEntry {
            boot: BOOT_INACTIVE,
            first_chs: CHS::empty(),
            sys: 0x06,
            last_chs: CHS::empty(),
            starting_lba: 1,
            sectors: mbr.disk_size - 1,
        };

        partition_byte_range = get_partition_byte_range(&mbr, sector_size);
        mbr.write_into(&mut cursor).unwrap();

        let mut cursor = StdIoWrapper::from(
            Cursor::new(&mut disk_image[partition_byte_range.clone()])
        );

        fatfs::format_volume(
            &mut cursor,
            FormatVolumeOptions::new().volume_label(*b"ShadeHelper"),
        ).unwrap();

        let cursor = Cursor::new(&mut disk_image[partition_byte_range]);
        let fs = FileSystem::new(
            cursor,
            FsOptions::new().update_accessed_date(false),
        ).unwrap();

        let root_dir = fs.root_dir();

        fn f<IO: fatfs::ReadWriteSeek, TP, OCC>(dir_path: &PathBuf, dir_fs: &Dir<IO, TP, OCC>)
            where OCC: fatfs::OemCpConverter, TP: fatfs::TimeProvider {
            for entry in fs_err::read_dir(dir_path).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_file() {
                    let mut file = dir_fs.create_file(
                        path.file_name().unwrap().to_str().unwrap()
                    ).unwrap();
                    let content = fs_err::read(path).unwrap();
                    file.write_all(content.as_slice()).unwrap();
                } else if path.is_dir() {
                    let subdir_fs = dir_fs.create_dir(
                        path.file_name().unwrap().to_str().unwrap()
                    ).unwrap();
                    f(&path, &subdir_fs)
                }
            }
        }
        f(from_folder, &root_dir);
    }
    fs_err::write(to_file, &disk_image).unwrap();
}

fn get_partition_byte_range(mbr: &MBR, sector_size: usize) -> Range<usize> {
    let partition_start_byte = mbr[1].starting_lba as usize * sector_size;
    let partition_num_bytes = mbr[1].sectors as usize * sector_size;
    partition_start_byte..partition_start_byte + partition_num_bytes
}
