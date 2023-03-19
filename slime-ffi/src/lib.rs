pub struct Common {

}

pub struct Jvm {

}

pub struct Dart {

}

pub enum AddonType {
    Common(Common),
    Jvm(Jvm),
    Dart(Dart),
    Napi,
    Python
}

pub trait Config {
    const ADDON_TYPE: AddonType;

    fn package() -> String;
}

pub struct Project {
    pub name: String,
    pub addon_types: Vec<AddonType>,
}