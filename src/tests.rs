pub fn init_logger() {
    let _ = env_logger::builder()
        // Include all events in tests
        .filter_level(log::LevelFilter::max())
        // Ignore errors initializing the logger if tests race to configure it
        .try_init();
}

pub struct TestRun {
    pub before: std::path::PathBuf,
    pub current: tempfile::TempDir,
    files: Option<Vec<std::path::PathBuf>>,
}

impl TestRun {
    pub fn init() -> anyhow::Result<Self> {
        let current_dir = std::env::current_dir()?;
        let temp_dir = tempfile::tempdir()?;
        std::env::set_current_dir(temp_dir.path())?;
        debug!(
            "Current dir: {}",
            temp_dir.path().as_os_str().to_string_lossy()
        );

        Ok(Self {
            before: current_dir,
            current: temp_dir,
            files: None,
        })
    }

    pub fn ls(folder: std::path::PathBuf) -> anyhow::Result<Vec<std::path::PathBuf>> {
        std::fs::read_dir(folder)?
            .map(|res| res.map(|f| f.path()))
            .collect::<Result<Vec<_>, ::std::io::Error>>()
            .map_err(anyhow::Error::from)
    }

    #[allow(unused)]
    pub fn read_file(file: std::path::PathBuf) -> anyhow::Result<Vec<u8>> {
        let mut content = Vec::new();
        let mut file = ::std::fs::File::open(file)?;

        use std::io::Read;
        file.read_to_end(&mut content)?;
        Ok(content)
    }

    pub fn update_files(&mut self) {
        let files = TestRun::ls(self.current.path().into()).unwrap();
        let _ = std::mem::replace(&mut self.files, Some(files.clone()));
        trace!("Files: {:#?}", files);
    }

    pub fn assert_has_file(&mut self, file: std::path::PathBuf) {
        match &self.files {
            Some(files) => assert!(
                files.contains(&file),
                format!(
                    "File {} not found!",
                    file.into_os_string().into_string().unwrap()
                )
            ),
            None => panic!(format!(
                "File {} not found",
                file.into_os_string().into_string().unwrap()
            )),
        }
    }

    pub fn reset(&self) -> anyhow::Result<()> {
        std::env::set_current_dir(&self.before)?;
        trace!(
            "Leaving {}",
            &self.current.path().as_os_str().to_string_lossy()
        );
        debug!(
            "Current dir: {}",
            &self.before.as_os_str().to_string_lossy()
        );
        Ok(())
    }
}

macro_rules! test {
    (
        name: $name:ident,
        temp_dir: true,
        steps: $steps:block
    ) => (
        test! {
            name: $name,
            temp_dir: true,
            vars: [],
            steps: $steps,
            assert: []
        }
    );
    (
        name: $name:ident,
        temp_dir: true,
        vars: [$($var:ident => $value:expr),*],
        steps: $steps:block,
        assert: [$($file:expr => $content:expr),*]
    ) => (
        #[test]
        #[serial(io)]
        fn $name() -> anyhow::Result<()> {
            crate::tests::init_logger();

            $(
                let $var = $value;
            )*
            #[allow(unused_mut)]
            let mut it = crate::tests::TestRun::init()?;

            $steps

            it.update_files();
            $(
                use std::io::prelude::*;

                it.assert_has_file(it.current.path().join($file));
                let mut created_content = Vec::new();
                let mut file = ::std::fs::File::open(&it.current.path().join($file)).unwrap();
                file.read_to_end(&mut created_content).unwrap();

                assert_eq!($content, created_content, "Equal check for {} failed!", $file);
            )*

            it.reset()?;
            Ok(())
        }
    );
    (
        name: $name:ident,
        steps: $steps:block
    ) => (
        #[test]
        fn $name() -> anyhow::Result<()> {
            crate::tests::init_logger();

            $steps

            Ok(())
        }
    );
}
