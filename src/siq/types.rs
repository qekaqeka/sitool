mod generated;
use std::io::Read;
use std::io::Seek;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

use chrono::DateTime;
use chrono::Utc;
use generated as internal_types;

use internal_types::InfoType as InternalInfo;
use internal_types::InfoTypeAuthorsElementType as InternalInfoAuthors;
use internal_types::InfoTypeSourcesElementType as InternalInfoSources;
use internal_types::Package as InternalPackage;
use internal_types::PackageGlobalAuthorsElementType as InternalGlobalAuthors;
use internal_types::PackageGlobalElementType as InternalGlobalInfo;
use internal_types::PackageGlobalSourcesElementType as InternalGlobalSources;
use internal_types::PackageRoundsElementType as InternalRounds;
use internal_types::PackageRoundsRoundElementType as InternalRound;
use internal_types::PackageRoundsRoundThemesElementType as InternalThemes;
use internal_types::PackageRoundsRoundThemesThemeElementType as InternalTheme;
use internal_types::PackageRoundsRoundThemesThemeQuestionsElementType as InternalQuestions;
use internal_types::PackageRoundsRoundThemesThemeQuestionsQuestionElementType as InternalQuestion;
use internal_types::PackageRoundsRoundThemesThemeQuestionsQuestionRightElementType as InternalAnswer;
use internal_types::PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomElementType as InternalAtom;
use internal_types::PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomType as InternalAtomType;
use internal_types::PackageRoundsRoundThemesThemeQuestionsQuestionScenarioElementType as InternalScenario;
use internal_types::PackageRoundsRoundThemesThemeQuestionsQuestionScriptElementType as InternalScript;
use internal_types::PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameElementType as InternalQuestionTypeName;
use internal_types::PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameParamElementType as InternalQuestionTypeParam;
use internal_types::PackageTagsElementType as InternalTags;
use internal_types::ParameterType as InternalParam;
use internal_types::ParameterTypeItemElementType as InternalParamItem;
use internal_types::ParameterTypeNumberSetElementType as InternalParamsNumberSet;
use internal_types::ParametersType as InternalParams;
use uuid::Uuid;
use xsd_parser_types::quick_xml::SerializeSync;
use xsd_parser_types::quick_xml::Writer;

use super::error::SiqError;

mod filedep;
use filedep::FileDep;

use xsd_parser_types::quick_xml::DeserializeSync;
use xsd_parser_types::quick_xml::XmlReaderSync;
use xsd_parser_types::xml::Mixed;
use xsd_parser_types::xml::Text;
use zip::ZipWriter;

use crate::siq::types::filedep::FileDepDstDir;

pub(super) trait SiqFacageElement<Internal>:
    TryFrom<Internal, Error = SiqError> + Into<Internal> + Clone
{
    fn bind_zip<P: AsRef<Path>>(&mut self, zip_path: P) {
        unimplemented!()
    }

    fn pack<W: Write + Seek>(&self, dest: &mut ZipWriter<W>) -> Result<(), SiqError> {
        unimplemented!()
    }
}

#[derive(Clone, Debug)]
pub struct Info {
    pub authors_names: Vec<String>,
    pub sources_strs: Vec<String>,
    pub comments: Option<String>,
    pub showman_comments: Option<String>,
    pub extension: Option<String>,
}

impl TryFrom<InternalInfo> for Info {
    type Error = SiqError;
    fn try_from(internal: InternalInfo) -> Result<Self, Self::Error> {
        let res = Self {
            authors_names: internal.authors.map_or(Vec::new(), |a| a.author),
            sources_strs: internal.sources.map_or(Vec::new(), |s| s.source),
            comments: internal.comments,
            showman_comments: internal.showman_comments,
            extension: internal.extension,
        };

        Ok(res)
    }
}

impl Into<InternalInfo> for Info {
    fn into(self) -> InternalInfo {
        InternalInfo {
            authors: Some(InternalInfoAuthors {
                author: self.authors_names,
            }),
            sources: Some(InternalInfoSources {
                source: self.sources_strs,
            }),
            comments: self.comments,
            showman_comments: self.showman_comments,
            extension: self.extension,
        }
    }
}

impl SiqFacageElement<InternalInfo> for Info {}

#[derive(Clone, Debug)]
pub struct Author {
    pub name: String,
    pub second_name: String,
    pub surname: String,
    pub country: String,
    pub city: String,
    pub id: String,
}

impl TryFrom<InternalGlobalAuthors> for Author {
    type Error = SiqError;
    fn try_from(internal: InternalGlobalAuthors) -> Result<Self, Self::Error> {
        let res = Self {
            name: internal.name,
            second_name: internal.second_name,
            surname: internal.surname,
            country: internal.country,
            city: internal.city,
            id: internal.id,
        };

        Ok(res)
    }
}

impl Into<InternalGlobalAuthors> for Author {
    fn into(self) -> InternalGlobalAuthors {
        InternalGlobalAuthors {
            name: self.name,
            second_name: self.second_name,
            surname: self.surname,
            country: self.country,
            city: self.city,
            id: self.id,
        }
    }
}

impl SiqFacageElement<InternalGlobalAuthors> for Author {}

#[derive(Clone, Debug)]
pub struct Source {
    pub author: String,
    pub title: String,
    pub year: String,
    pub publish: String,
    pub city: String,
    pub id: String,
}

impl TryFrom<InternalGlobalSources> for Source {
    type Error = SiqError;
    fn try_from(internal: InternalGlobalSources) -> Result<Self, Self::Error> {
        let res = Self {
            author: internal.author,
            title: internal.title,
            year: internal.year,
            publish: internal.publish,
            city: internal.city,
            id: internal.id,
        };

        Ok(res)
    }
}

impl Into<InternalGlobalSources> for Source {
    fn into(self) -> InternalGlobalSources {
        InternalGlobalSources {
            author: self.author,
            title: self.title,
            year: self.year,
            publish: self.publish,
            city: self.city,
            id: self.id,
        }
    }
}

impl SiqFacageElement<InternalGlobalSources> for Source {}

#[derive(Clone, Debug)]
pub struct GlobalInfo {
    pub author: Author,
    pub source: Source,
}

impl TryFrom<InternalGlobalInfo> for GlobalInfo {
    type Error = SiqError;
    fn try_from(internal: InternalGlobalInfo) -> Result<Self, Self::Error> {
        let res = Self {
            author: Author::try_from(internal.authors)?,
            source: Source::try_from(internal.sources)?,
        };

        Ok(res)
    }
}

impl Into<InternalGlobalInfo> for GlobalInfo {
    fn into(self) -> InternalGlobalInfo {
        InternalGlobalInfo {
            authors: self.author.into(),
            sources: self.source.into(),
        }
    }
}

impl SiqFacageElement<InternalGlobalInfo> for GlobalInfo {}

#[derive(Clone, Debug)]
pub struct OldQuestionTypeParam {
    pub name: String,
    pub content: String,
}

impl TryFrom<InternalQuestionTypeParam> for OldQuestionTypeParam {
    type Error = SiqError;
    fn try_from(internal: InternalQuestionTypeParam) -> Result<Self, SiqError> {
        let res = Self {
            name: internal.name,
            content: internal.content,
        };

        Ok(res)
    }
}

impl Into<InternalQuestionTypeParam> for OldQuestionTypeParam {
    fn into(self) -> InternalQuestionTypeParam {
        InternalQuestionTypeParam {
            name: self.name,
            content: self.content,
        }
    }
}

impl SiqFacageElement<InternalQuestionTypeParam> for OldQuestionTypeParam {}

#[derive(Clone, Debug)]
pub struct OldQuestionType {
    pub name: String,
    pub params: Vec<OldQuestionTypeParam>,
}

impl TryFrom<InternalQuestionTypeName> for OldQuestionType {
    type Error = SiqError;
    fn try_from(internal: InternalQuestionTypeName) -> Result<Self, Self::Error> {
        let res = Self {
            name: internal.name,
            params: internal
                .param
                .into_iter()
                .map(|p| p.try_into())
                .collect::<Result<Vec<OldQuestionTypeParam>, SiqError>>()?,
        };

        Ok(res)
    }
}

impl Into<InternalQuestionTypeName> for OldQuestionType {
    fn into(self) -> InternalQuestionTypeName {
        InternalQuestionTypeName {
            name: self.name,
            param: self.params.into_iter().map(|p| p.into()).collect(),
        }
    }
}

impl SiqFacageElement<InternalQuestionTypeName> for OldQuestionType {}

// Exists because InternalAtomType doesn't implement Clone, Debug trait
#[derive(Clone, Debug)]
pub enum AtomType {
    Text,
    Say,
    Image,
    Voice,
    Video,
    Marker,
}

impl From<InternalAtomType> for AtomType {
    fn from(value: InternalAtomType) -> Self {
        match value {
            InternalAtomType::Text => Self::Text,
            InternalAtomType::Say => Self::Say,
            InternalAtomType::Image => Self::Image,
            InternalAtomType::Voice => Self::Voice,
            InternalAtomType::Video => Self::Video,
            InternalAtomType::Marker => Self::Marker,
        }
    }
}

impl Into<InternalAtomType> for AtomType {
    fn into(self) -> InternalAtomType {
        match self {
            Self::Text => InternalAtomType::Text,
            Self::Say => InternalAtomType::Say,
            Self::Image => InternalAtomType::Image,
            Self::Voice => InternalAtomType::Voice,
            Self::Video => InternalAtomType::Video,
            Self::Marker => InternalAtomType::Marker,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Atom {
    pub type_: Option<AtomType>,
    pub time: Option<f64>,
    pub content: String,

    filedep: Option<FileDep>,
}

impl TryFrom<InternalAtom> for Atom {
    type Error = SiqError;
    fn try_from(internal: InternalAtom) -> Result<Self, Self::Error> {
        let atom_type = internal.type_.map(|t| t.into());
        let mut filedep = None;

        if let Some(at) = atom_type.as_ref() && internal.content.starts_with('@') {
            let filename = &internal.content[1..];
            match at {
                AtomType::Image => {
                    let path = Path::new("Images").join(filename);
                    filedep = Some(FileDep::new_compressed(path, FileDepDstDir::Images));
                },
                AtomType::Video => {
                    let path = Path::new("Video").join(filename);
                    filedep = Some(FileDep::new_compressed(path, FileDepDstDir::Video));
                },
                AtomType::Voice => {
                    let path = Path::new("Voice").join(filename);
                    filedep = Some(FileDep::new_compressed(path, FileDepDstDir::Audio));
                },
                _ => (),
            }
        }

        let res = Self {
            type_: atom_type,
            time: internal.time,
            content: internal.content,

            filedep: filedep,
        };

        Ok(res)
    }
}

impl Into<InternalAtom> for Atom {
    fn into(self) -> InternalAtom {
        let content = if let Some(fd) = self.filedep {
            let filename = fd.get_dst_path()
                .file_name()
                .expect("file")
                .to_str()
                .expect("UTF-8")
                .to_string();

            format!("@{}", filename)
        } else {
            self.content
        };

        InternalAtom {
            type_: self.type_.map(|t| t.into()),
            time: self.time,
            content: content,
        }
    }
}

impl SiqFacageElement<InternalAtom> for Atom {
    fn bind_zip<P: AsRef<Path>>(&mut self, zip_path: P) {
        self.filedep.as_mut()
            .map(|fd| fd.bind_zip_path(zip_path));
    }

    fn pack<W: Write + Seek>(&self, dest: &mut ZipWriter<W>) -> Result<(), SiqError> {
        if let Some(fd) = self.filedep.as_ref() {
            fd.pack(dest)
        } else {
            Ok(())
        }
    }
}

#[derive(Clone, Debug)]
pub struct Scenario {
    atoms: Vec<Atom>,
}

impl TryFrom<InternalScenario> for Scenario {
    type Error = SiqError;
    fn try_from(internal: InternalScenario) -> Result<Self, Self::Error> {
        let res = Self {
            atoms: internal
                .atom
                .into_iter()
                .map(|atom| atom.try_into())
                .collect::<Result<Vec<Atom>, SiqError>>()?,
        };

        Ok(res)
    }
}

impl Into<InternalScenario> for Scenario {
    fn into(self) -> InternalScenario {
        InternalScenario {
            atom: self.atoms.into_iter().map(|atom| atom.into()).collect(),
        }
    }
}

impl SiqFacageElement<InternalScenario> for Scenario {
    fn bind_zip<P: AsRef<Path>>(&mut self, zip_path: P) {
        self.atoms.iter_mut()
            .for_each(|atom| atom.bind_zip(zip_path.as_ref()));
    }

    fn pack<W: Write + Seek>(&self, dest: &mut ZipWriter<W>) -> Result<(), SiqError> {
        self.atoms.iter()
            .try_for_each(|atom| atom.pack(dest))
    }
}

#[derive(Clone, Debug)]
enum ParamItemType {
    Image,
    Video,
    Audio,
}

impl FromStr for ParamItemType {
    type Err = SiqError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "image" => Ok(Self::Image),
            "video" => Ok(Self::Video),
            "audio" => Ok(Self::Audio),
            _ => Err(Self::Err::FailedToConvert("bad param item type")),
        }
    }
}

impl Into<FileDepDstDir> for ParamItemType {
    fn into(self) -> FileDepDstDir {
        match self {
            Self::Audio => FileDepDstDir::Audio,
            Self::Video => FileDepDstDir::Video,
            Self::Image => FileDepDstDir::Images,
        }
    }
}

impl ToString for ParamItemType {
    fn to_string(&self) -> String {
        let res = match self {
            Self::Audio => "audio",
            Self::Image => "image",
            Self::Video => "video",
        };

        res.to_string()
    }
}

impl ParamItemType {
    fn to_dir_name(&self) -> &Path {
        let res = match self {
            Self::Audio => "Audio",
            Self::Image => "Images",
            Self::Video => "Video",
        };

        Path::new(res)
    }
}

#[derive(Clone, Debug)]
enum ParamItemPlacement {
    Screen,
    Replic,
    Background,
}

impl FromStr for ParamItemPlacement {
    type Err = SiqError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ("screen") => Ok(Self::Screen),
            ("replic") => Ok(Self::Replic),
            ("background") => Ok(Self::Background),
            _ => Err(Self::Err::FailedToConvert("bad param item placement")),
        }
    }
}

impl ToString for ParamItemPlacement {
    fn to_string(&self) -> String {
        let res = match self {
            Self::Screen => "screen",
            Self::Replic => "replic",
            Self::Background => "background",
        };

        res.to_string()
    }
}

#[derive(Clone, Debug)]
struct ParamItem {
    type_: Option<ParamItemType>,
    is_ref: bool,
    placement: Option<ParamItemPlacement>,
    duration: Option<String>,
    wait_for_finish: bool,
    content: String,

    filedep: Option<FileDep>,
}

impl TryFrom<InternalParamItem> for ParamItem {
    type Error = SiqError;
    fn try_from(internal: InternalParamItem) -> Result<Self, Self::Error> {
        let placement = if let Some(pl) = internal.placement {
            Some(ParamItemPlacement::from_str(&pl)?)
        } else {
            // Screen is the default placement
            None
        };

        let type_ = if let Some(tp) = internal.type_ {
            Some(ParamItemType::from_str(&tp)?)
        } else {
            None
        };

        let is_ref = internal.is_ref.is_some_and(|s| s == "True");

        let filedep = if is_ref {
            let dst_type: FileDepDstDir = type_
                .as_ref()
                .expect("should have type, because has is_ref == true")
                .to_owned()
                .into();

            let mut inner_path = type_
                .as_ref()
                .expect("should have type, because has is_ref == true")
                .to_dir_name()
                .to_path_buf();

            // content has file
            inner_path.push(&internal.content);
            Some(FileDep::new_compressed(&inner_path, dst_type))
        } else {
            None
        };

        let res = Self {
            type_: type_,
            is_ref: is_ref,
            placement: placement,
            duration: internal.duration,
            wait_for_finish: internal.wait_for_finish.is_some_and(|s| s == "True"),
            content: internal.content,

            filedep: filedep,
        };

        Ok(res)
    }
}

impl Into<InternalParamItem> for ParamItem {
    fn into(self) -> InternalParamItem {
        let content = if let Some(fd) = self.filedep {
            fd.get_dst_path()
                .file_name()
                .expect("file")
                .to_str()
                .expect("UUID is UTF-8")
                .to_string()
        } else {
            self.content
        };

        InternalParamItem {
            type_: self.type_.map(|t| t.to_string()),
            is_ref: self.is_ref.then_some("True".to_string()),
            placement: self.placement.map(|pl| pl.to_string()),
            duration: self.duration,
            wait_for_finish: self.wait_for_finish.then_some("True".to_string()),
            content: content,
        }
    }
}

impl SiqFacageElement<InternalParamItem> for ParamItem {
    fn bind_zip<P: AsRef<Path>>(&mut self, zip_path: P) {
        if let Some(fd) = self.filedep.as_mut() {
            fd.bind_zip_path(zip_path);
        }
    }

    fn pack<W: Write + Seek>(&self, dest: &mut ZipWriter<W>) -> Result<(), SiqError> {
        if let Some(fd) = self.filedep.as_ref() {
            fd.pack(dest)
        } else {
            Ok(())
        }
    }
}

#[derive(Clone, Debug)]
struct NumberSet {
    minimum: Option<i32>,
    maximum: Option<i32>,
    step: Option<i32>,
    content: String,
}

impl TryFrom<InternalParamsNumberSet> for NumberSet {
    type Error = SiqError;
    fn try_from(internal: InternalParamsNumberSet) -> Result<Self, Self::Error> {
        let res = Self {
            minimum: internal.minimum,
            maximum: internal.maximum,
            step: internal.step,
            content: internal.content,
        };

        Ok(res)
    }
}

impl Into<InternalParamsNumberSet> for NumberSet {
    fn into(self) -> InternalParamsNumberSet {
        InternalParamsNumberSet {
            minimum: self.minimum,
            maximum: self.maximum,
            step: self.step,
            content: self.content,
        }
    }
}

impl SiqFacageElement<InternalParamsNumberSet> for NumberSet {}

#[derive(Clone, Debug)]
enum ParamType {
    Simple,
    Content,
    Group,
    NumberSet,
}

impl FromStr for ParamType {
    type Err = SiqError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ("" | "simple") => Ok(Self::Simple),
            "content" => Ok(Self::Content),
            "group" => Ok(Self::Group),
            "numberSet" => Ok(Self::NumberSet),
            _ => Err(Self::Err::FailedToConvert("bad param type")),
        }
    }
}

impl ToString for ParamType {
    fn to_string(&self) -> String {
        let res = match self {
            Self::Simple => "simple",
            Self::Content => "content",
            Self::Group => "group",
            Self::NumberSet => "numberSet",
        };

        res.to_string()
    }
}

#[derive(Clone, Debug)]
struct Param {
    name: Option<String>,
    type_: Option<ParamType>,
    text_before: Option<String>,
    items: Vec<ParamItem>,
    params: Vec<Box<Param>>,
    number_set: Option<NumberSet>,
}

impl TryFrom<InternalParam> for Param {
    type Error = SiqError;
    fn try_from(internal: InternalParam) -> Result<Self, Self::Error> {
        let number_set = if let Some(ns) = internal.number_set {
            Some(ns.value.try_into()?)
        } else {
            None
        };

        let type_ = if let Some(s) = internal.type_ {
            Some(ParamType::from_str(&s)?)
        } else {
            None
        };

        let res = Self {
            name: internal.name,
            type_: type_,
            text_before: internal.text_before.map(|t| t.0),
            items: internal
                .item
                .into_iter()
                .map(|it| it.value.try_into())
                .collect::<Result<Vec<ParamItem>, SiqError>>()?,
            params: internal
                .param
                .into_iter()
                .map(|p| p.value.try_into().map(Box::new))
                .collect::<Result<Vec<Box<Param>>, SiqError>>()?,
            number_set: number_set,
        };

        Ok(res)
    }
}

impl Into<InternalParam> for Param {
    fn into(self) -> InternalParam {
        InternalParam {
            name: self.name,
            type_: self.type_.map(|t| t.to_string()),
            text_before: self.text_before.map(|tb| Text::new(tb)),
            item: self
                .items
                .into_iter()
                .map(|item| Mixed::<InternalParamItem>::new(item.into()))
                .collect(),
            param: self
                .params
                .into_iter()
                .map(|param| Mixed::<InternalParam>::new((*param).into()))
                .collect(),
            number_set: self
                .number_set
                .map(|ns| Mixed::<InternalParamsNumberSet>::new(ns.into())),
        }
    }
}

impl SiqFacageElement<InternalParam> for Param {
    fn bind_zip<P: AsRef<Path>>(&mut self, zip_path: P) {
        // get https://github.com/rust-lang/rust/issues/43520 if you use for_each here
        for param in self.params.iter_mut() {
            param.bind_zip(zip_path.as_ref());
        }

        self.items.iter_mut().for_each(|it| it.bind_zip(zip_path.as_ref()));
    }

    fn pack<W: Write + Seek>(&self, dest: &mut ZipWriter<W>) -> Result<(), SiqError> {
        // get https://github.com/rust-lang/rust/issues/43520 if you use for_each here
        for param in self.params.iter() {
            param.pack(dest)?;
        }

        self.items.iter()
            .try_for_each(|p| p.pack(dest))
    }
}

#[derive(Clone, Debug)]
struct Script {
    steps: Vec<Vec<Param>>,
}

impl TryFrom<InternalScript> for Script {
    type Error = SiqError;
    fn try_from(internal: InternalScript) -> Result<Self, Self::Error> {
        let res = Self {
            steps: internal
                .step
                .into_iter()
                .map(|ps| {
                    ps.param
                        .into_iter()
                        .map(|p| p.try_into())
                        .collect::<Result<Vec<Param>, SiqError>>()
                })
                .collect::<Result<Vec<Vec<Param>>, SiqError>>()?,
        };

        Ok(res)
    }
}

impl Into<InternalScript> for Script {
    fn into(self) -> InternalScript {
        InternalScript {
            step: self
                .steps
                .into_iter()
                .map(|step| {
                    let internal_step = step.into_iter().map(|s| s.into()).collect();

                    InternalParams {
                        param: internal_step,
                    }
                })
                .collect(),
        }
    }
}

impl SiqFacageElement<InternalScript> for Script {
    fn bind_zip<P: AsRef<Path>>(&mut self, zip_path: P) {
        for step in self.steps.iter_mut() {
            for param in step.iter_mut() {
                param.bind_zip(zip_path.as_ref());
            }
        }
    }

    fn pack<W: Write + Seek>(&self, dest: &mut ZipWriter<W>) -> Result<(), SiqError> {
        self.steps.iter().flatten()
            .try_for_each(|p| p.pack(dest))
    }
}

#[derive(Clone, Debug)]
pub struct Question {
    pub info: Option<Info>,
    pub right_answers: Vec<String>,
    pub wrong_answers: Vec<String>,
    pub price: i32,
    pub scenario: Option<Scenario>,
    pub type_name: Option<OldQuestionType>,
    type_: Option<String>,
    params: Vec<Param>,
    script: Option<Script>,
}

impl TryFrom<InternalQuestion> for Question {
    type Error = SiqError;
    fn try_from(internal: InternalQuestion) -> Result<Self, Self::Error> {
        let info = if let Some(ii) = internal.info {
            Some(ii.try_into()?)
        } else {
            None
        };

        let params = if let Some(ps) = internal.params {
            ps.param
                .into_iter()
                .map(|p| p.try_into())
                .collect::<Result<Vec<Param>, SiqError>>()?
        } else {
            Vec::new()
        };

        let scenario = if let Some(sc) = internal.scenario {
            Some(sc.try_into()?)
        } else {
            None
        };

        let type_name = if let Some(tn) = internal.type_name {
            Some(tn.try_into()?)
        } else {
            None
        };

        let script = if let Some(sc) = internal.script {
            Some(sc.try_into()?)
        } else {
            None
        };

        let res = Self {
            info: info,
            right_answers: internal.right.answer,
            wrong_answers: internal.wrong.map_or(Vec::new(), |wa| wa.answer),
            price: internal.price,
            type_: internal.type_,
            scenario: scenario,
            params: params,
            type_name: type_name,
            script: script,
        };

        Ok(res)
    }
}

impl Into<InternalQuestion> for Question {
    fn into(self) -> InternalQuestion {
        let params = self.params.into_iter().map(|p| p.into()).collect();

        InternalQuestion {
            info: self.info.map(|i| i.into()),
            right: InternalAnswer {
                answer: self.right_answers,
            },
            wrong: Some(InternalAnswer {
                answer: self.wrong_answers,
            }),
            price: self.price,
            type_: self.type_,
            params: Some(InternalParams { param: params }),
            scenario: self.scenario.map(|sc| sc.into()),
            type_name: self.type_name.map(|tn| tn.into()),
            script: self.script.map(|sc| sc.into()),
        }
    }
}

impl SiqFacageElement<InternalQuestion> for Question {
    fn bind_zip<P: AsRef<Path>>(&mut self, zip_path: P) {
        self.script.as_mut()
            .map(|sc| sc.bind_zip(zip_path.as_ref()));

        self.params.iter_mut()
            .for_each(|p| p.bind_zip(zip_path.as_ref()));

        self.scenario.as_mut()
            .map(|scn| scn.bind_zip(zip_path.as_ref()));
    }

    fn pack<W: Write + Seek>(&self, dest: &mut ZipWriter<W>) -> Result<(), SiqError> {
        if let Some(sc) = self.script.as_ref() {
            sc.pack(dest)?;
        }

        if let Some(scn) = self.scenario.as_ref() {
            scn.pack(dest)?;
        }

        self.params.iter()
            .try_for_each(|p| p.pack(dest))
    }
}

#[derive(Clone, Debug)]
pub struct Theme {
    pub info: Option<Info>,
    pub name: String,
    pub questions: Vec<Question>,
}

impl TryFrom<InternalTheme> for Theme {
    type Error = SiqError;
    fn try_from(internal: InternalTheme) -> Result<Self, Self::Error> {
        let info = if let Some(ii) = internal.info {
            Some(ii.try_into()?)
        } else {
            None
        };

        let questions = if let Some(qs) = internal.questions {
            qs.question
                .into_iter()
                .map(Question::try_from)
                .collect::<Result<Vec<Question>, SiqError>>()?
        } else {
            Vec::new()
        };

        let res = Self {
            info: info,
            name: internal.name,
            questions: questions,
        };

        Ok(res)
    }
}

impl Into<InternalTheme> for Theme {
    fn into(self) -> InternalTheme {
        let questions = self.questions.into_iter().map(|q| q.into()).collect();

        InternalTheme {
            info: self.info.map(|i| i.into()),
            name: self.name,
            questions: Some(InternalQuestions {
                question: questions,
            }),
        }
    }
}

impl SiqFacageElement<InternalTheme> for Theme {
    fn bind_zip<P: AsRef<Path>>(&mut self, zip_path: P) {
        self.questions.iter_mut()
            .for_each(|q| q.bind_zip(zip_path.as_ref()));
    }

    fn pack<W: Write + Seek>(&self, dest: &mut ZipWriter<W>) -> Result<(), SiqError> {
        self.questions.iter()
            .try_for_each(|q| q.pack(dest))
    }
}

#[derive(Clone, Debug)]
pub enum RoundType {
    Common(String),
    Final,
}

impl FromStr for RoundType {
    type Err = SiqError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.as_ref() {
            "final" => Ok(Self::Final),
            _ => Ok(Self::Common(value.to_string())),
        }
    }
}

impl RoundType {
    fn into_internal_round_type(self) -> String {
        match self {
            Self::Final => "final".to_string(),
            Self::Common(s) => s,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Round {
    pub info: Option<Info>,
    pub name: String,
    pub type_: Option<RoundType>,
    pub themes: Vec<Theme>,
}

impl TryFrom<InternalRound> for Round {
    type Error = SiqError;
    fn try_from(internal: InternalRound) -> Result<Self, Self::Error> {
        let info = if let Some(ii) = internal.info {
            Some(ii.try_into()?)
        } else {
            None
        };

        let themes = if let Some(ts) = internal.themes {
            ts.theme
                .into_iter()
                .map(|t| t.try_into())
                .collect::<Result<Vec<Theme>, SiqError>>()?
        } else {
            Vec::new()
        };

        let round_type = if let Some(rt) = internal.type_ {
            Some(RoundType::from_str(&rt)?)
        } else {
            None
        };

        let res = Self {
            info: info,
            name: internal.name,
            type_: round_type,
            themes: themes,
        };

        Ok(res)
    }
}

impl Into<InternalRound> for Round {
    fn into(self) -> InternalRound {
        let themes = self.themes.into_iter().map(|theme| theme.into()).collect();

        InternalRound {
            info: self.info.map(|i| i.into()),
            name: self.name,
            type_: self.type_.map(|t| t.into_internal_round_type()),
            themes: Some(InternalThemes { theme: themes }),
        }
    }
}

impl SiqFacageElement<InternalRound> for Round {
    fn bind_zip<P: AsRef<Path>>(&mut self, zip_path: P) {
        self.themes.iter_mut().for_each(|t| t.bind_zip(zip_path.as_ref()));
    }

    fn pack<W: Write + Seek>(&self, dest: &mut ZipWriter<W>) -> Result<(), SiqError> {
        self.themes.iter()
            .try_for_each(|t| t.pack(dest))
    }
}

impl Default for Round {
    fn default() -> Self {
        Self {
            info: None,
            name: String::default(),
            type_: None,
            themes: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Package {
    pub info: Option<Info>,
    pub rounds: Vec<Round>,
    pub tags: Vec<String>,
    pub global: Option<GlobalInfo>,
    pub id: Option<String>,
    pub name: String,
    pub version: f64,
    pub restriction: Option<String>,
    pub date: Option<String>,
    pub publisher: Option<String>,
    pub difficulty: Option<u32>,
    pub language: Option<String>,
    pub generator: Option<String>,
    pub contact_uri: Option<String>,
    logo: Option<String>,

    logo_link: Option<FileDep>,
}

impl TryFrom<InternalPackage> for Package {
        type Error = SiqError;
    fn try_from(internal: InternalPackage) -> Result<Self, Self::Error> {
        let info = if let Some(ii) = internal.info {
            Some(ii.try_into()?)
        } else {
            None
        };

        let rounds = if let Some(rs) = internal.rounds {
            rs.round
                .into_iter()
                .map(|r| r.try_into())
                .collect::<Result<Vec<Round>, SiqError>>()?
        } else {
            Vec::new()
        };

        let global = if let Some(gl) = internal.global {
            Some(gl.try_into()?)
        } else {
            None
        };

        let logo_link = if let Some(logo_name) = internal.logo.as_ref() {
            if logo_name.starts_with('@') {
                // logo is stored in Images dir
                let mut inner_path = PathBuf::from("Images");
                inner_path.push(&logo_name[1..]);

                Some(FileDep::new_compressed(&inner_path, FileDepDstDir::Images))
            } else {
                None
            }
        } else {
            None
        };

        let res = Self {
            info: info,
            rounds: rounds,
            tags: internal.tags.map_or(Vec::new(), |tag| tag.tag),
            global: global,
            id: internal.id,
            name: internal.name,
            version: internal.version,
            restriction: internal.restriction,
            date: internal.date,
            publisher: internal.publisher,
            difficulty: internal.difficulty,
            language: internal.language,
            generator: internal.generator,
            contact_uri: internal.contact_uri,
            logo: internal.logo,

            logo_link: logo_link,
        };

        Ok(res)
    }
}

impl Into<InternalPackage> for Package {
    fn into(self) -> InternalPackage {
        let rounds = self.rounds.into_iter().map(|round| round.into()).collect();

        let logo = if let Some(fd) = self.logo_link {
            let file_name = fd.get_dst_path()
                .file_name()
                .expect("logo is file")
                .to_str()
                .expect("UUID is UTF-8")
                .to_string();

            Some(format!("@{}", file_name))
        } else {
            self.logo
        };

        InternalPackage {
            info: self.info.map(|i| i.into()),
            rounds: Some(InternalRounds { round: rounds }),
            tags: Some(InternalTags { tag: self.tags }),
            global: self.global.map(|g| g.into()),
            id: self.id,
            name: self.name,
            version: self.version,
            restriction: self.restriction,
            date: self.date,
            publisher: self.publisher,
            difficulty: self.difficulty,
            language: self.language,
            generator: self.generator,
            contact_uri: self.contact_uri,
            logo: logo,
        }
    }
}

impl SiqFacageElement<InternalPackage> for Package {
    fn bind_zip<P: AsRef<Path>>(&mut self, zip_path: P) {
        self.logo_link.as_mut()
            .map(|ll| ll.bind_zip_path(zip_path.as_ref()));

        self.rounds.iter_mut()
            .for_each(|r| r.bind_zip(zip_path.as_ref()));
    }

    fn pack<W: Write + Seek>(&self, dest: &mut ZipWriter<W>) -> Result<(), SiqError> {
        if let Some(ll) = &self.logo_link {
            ll.pack(dest)?;
        }

        self.rounds.iter()
            .try_for_each(|r| r.pack(dest))
    }
}

impl Default for Package {
    fn default() -> Self {
        let info = Info {
            authors_names: vec!["sitool".to_string()],
            sources_strs: Vec::new(),
            comments: None,
            showman_comments: None,
            extension: None,
        };

        Self {
            info: Some(info),
            rounds: Vec::new(),
            tags: Vec::new(),
            global: None,
            id: Some(Uuid::new_v4().to_string()),
            name: String::default(),
            version: 5f64,
            restriction: None,
            date: Some(Utc::now().format("%d.%m.%Y").to_string()),
            publisher: None,
            difficulty: Some(1),
            language: Some("en-US".into()),
            generator: Some("generator".into()),
            contact_uri: None,
            logo: None,

            logo_link: None,
        }
    }
}

impl Package {
    pub fn set_logo<P: AsRef<Path>>(&mut self, path: P) {
        self.logo_link = Some(FileDep::new_plain(&path, FileDepDstDir::Images));
        let path_str = path.as_ref().as_os_str().to_string_lossy();
        self.logo = Some(format!("@{}", path_str));
    }

    pub fn get_logo(&self) -> Option<&String> {
        self.logo.as_ref()
    }

    pub(super) fn deserialize<'a, R: XmlReaderSync<'a>>(reader: &mut R) -> Result<Self, SiqError> {
        let internal = InternalPackage::deserialize(reader)?;

        let package = internal.try_into()?;

        Ok(package)
    }

    pub(super) fn serialize<W: Write>(&self, writer: &mut Writer<W>) -> Result<(), SiqError> {
        let internal: InternalPackage = self.clone().into();

        internal.serialize("package", writer)?;

        Ok(())
    }
}
