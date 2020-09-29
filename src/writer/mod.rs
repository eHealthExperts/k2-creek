use crate::api::Api;
use promptly::Promptable;

mod encode;
pub(crate) mod files;
pub(crate) mod results;

pub trait Write {
    fn write(&self) -> anyhow::Result<()>;
}

pub trait WriteApi {
    fn write(&self, force_delete: bool) -> anyhow::Result<()>;
}

impl WriteApi for Api {
    fn write(&self, force_delete: bool) -> anyhow::Result<()> {
        if files::Files::present() {
            if force_delete
                || bool::prompt_default(
                    "WARNING - Old files found in output folder. Delete before proceeding?",
                    false,
                )
                .unwrap_or(false)
            {
                files::Files::cleanup()
            } else {
                println!("Continuing with file generation. You will probably end up with an inconsistent set of result files.");
            }
        }

        match self {
            Api::V1(data) => data.write(),
            Api::V2(data) => data.write(),
            Api::V3(data) => data.write(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tests::TestRun;
    use test_server::helper;

    test! {
        name: not_prompting_before_cleanup,
        temp_dir: true,
        vars: [
            old_files => TestRun::ls("tests/writer/egk".into())?,
            kvk_res => helper::read_file("tests/api/v1/kvk.json")?
        ],
        it: it,
        steps: {
            for file in old_files {
                let base = it.before.as_path();
                let mut source = base.join("tests/writer/egk");
                let filename = &file.as_path().file_name().unwrap();
                source.push(filename);
                std::fs::copy(&source, &it.current.path().join(filename))?;
            }

            let api: Api = serde_json::from_str(&kvk_res)?;
            api.write(true).unwrap();

            it.update_files();
            for file in &["KVK_Daten.bin", "KVK.dat", "Result.xml"] {
                it.assert_has_file(it.current.path().join(file));
            }
        },
        assert: []
    }
}
