use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use serde_json::Value;

/// 読み込んだpackage.jsonからスクリプトを取り出す
/// # Errors
/// TBD
pub fn extract_scripts(json_string: &str) -> HashMap<String, String> {
  let maybe_json = serde_json::from_str(json_string);
  let json: Value = match maybe_json {
    Ok(result) => result,
    Err(e) => panic!("Could not parse json ... {}", e.to_string())
  };

  // 読み込んだJSON文字列から、値のコピーをとってHashMapの複製を作る
  let scripts = if let Value::Object(map) = &json["scripts"] {
    let mut scripts_map: HashMap<String, String> = HashMap::new();
    for (key, value) in map {
      // package.jsonのscriptsセクションはkey, valueともにstringであるはずなので他の型の場合は無視する
      if let Value::String(v) = value {
        scripts_map.insert(key.to_string(), v.to_owned());
      }
    }
    scripts_map
  } else {
    panic!("scripts in package.json is not a Map object.")
  };

  scripts
}

/// 引数のディレクトリ配下にある package.json を読み出して文字列にする
pub fn get_package_json_str(dir: &str) -> Result<String, std::io::Error> {
  let package_json = format!("{}/package.json", &dir);

  let file = File::open(package_json);
  let mut buffer = String::from("");
  match file {
    Ok(mut f) => {
      let _ = f.read_to_string(&mut buffer);
    },
    Err(e) => panic!("Cannot read package.json. Details: {}", e)
  }

  Ok(buffer)
}
