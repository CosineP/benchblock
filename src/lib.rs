#![feature(const_if_match)]

#[allow(dead_code)]
enum FetchType {
    Big,
    Tiny,
    Local,
}
// recommend local to prevent server timeouts
const FETCH_TYPE: FetchType = FetchType::Local;

pub const FETCH: &'static str = match FETCH_TYPE {
    FetchType::Big => "https://cdn.discordapp.com/attachments/424366350434041866/521924972818726914/b95d2b0.jpg",
    FetchType::Tiny => "https://gist.githubusercontent.com/CosineP/2eee57984c138ab472915099d99f9b34/raw/5a08ac6665dc6386d793877be250cbeffdfe1956/peridot",
    FetchType::Local => "http://localhost:8000/journal.txt",
};

