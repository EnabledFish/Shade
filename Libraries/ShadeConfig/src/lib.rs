use crate::file::ShadeConfigFile;

pub mod file;
pub mod fields;
pub mod object;
pub mod std;
pub mod path;

#[test]
fn test() {
    let mut file = ShadeConfigFile::default();
    file.parse(format!(r#"
        # Preinit field. (Can be emitted)
        System.Display = null

        # Set integer fields.
        System.Display.Resolution.Width = 1280

        # Set null fields.
        System.Display.Resolution.Height = null

        # Set string fields.
        System.Display.Name = "Common Display"

        # Set boolean fields.
        System.Display.AutoDetectResolution = false
    "#)).expect("Parser error.");
    println!("{:?}", file);
    println!("{:?}", file.get_root()
        .get_path(
            format!("System.Display.Resolution")
                .try_into().unwrap()
        ).unwrap()
    )
}
