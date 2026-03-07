pub const NS_XS: xsd_parser_types::misc::Namespace =
    xsd_parser_types::misc::Namespace::new_const(b"http://www.w3.org/2001/XMLSchema");
pub const NS_XML: xsd_parser_types::misc::Namespace =
    xsd_parser_types::misc::Namespace::new_const(b"http://www.w3.org/XML/1998/namespace");
pub const NS_UNNAMED_2: xsd_parser_types::misc::Namespace =
    xsd_parser_types::misc::Namespace::new_const(
        b"https://github.com/VladimirKhil/SI/blob/master/assets/siq_5.xsd",
    );
pub const PREFIX_XS: xsd_parser_types::misc::NamespacePrefix =
    xsd_parser_types::misc::NamespacePrefix::new_const(b"xs");
pub const PREFIX_XML: xsd_parser_types::misc::NamespacePrefix =
    xsd_parser_types::misc::NamespacePrefix::new_const(b"xml");
#[derive(Debug)]
pub struct InfoType {
    pub authors: ::core::option::Option<InfoTypeAuthorsElementType>,
    pub sources: ::core::option::Option<InfoTypeSourcesElementType>,
    pub comments: ::core::option::Option<::std::string::String>,
    pub showman_comments: ::core::option::Option<::std::string::String>,
    pub extension: ::core::option::Option<::std::string::String>,
}
impl ::xsd_parser_types::quick_xml::WithSerializer for InfoType {
    type Serializer<'x> = quick_xml_serialize::InfoTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: ::core::option::Option<&'ser ::core::primitive::str>,
        is_root: ::core::primitive::bool,
    ) -> ::core::result::Result<Self::Serializer<'ser>, ::xsd_parser_types::quick_xml::Error> {
        Ok(quick_xml_serialize::InfoTypeSerializer {
            value: self,
            state: ::std::boxed::Box::new(quick_xml_serialize::InfoTypeSerializerState::Init__),
            name: name.unwrap_or("infoType"),
            is_root,
        })
    }
}
impl ::xsd_parser_types::quick_xml::WithDeserializer for InfoType {
    type Deserializer = quick_xml_deserialize::InfoTypeDeserializer;
}
pub type Package = PackageElementType;
#[derive(Debug)]
pub struct PackageElementType {
    pub id: ::core::option::Option<::std::string::String>,
    pub name: ::std::string::String,
    pub version: ::core::primitive::f64,
    pub restriction: ::core::option::Option<::std::string::String>,
    pub date: ::core::option::Option<::std::string::String>,
    pub publisher: ::core::option::Option<::std::string::String>,
    pub difficulty: ::core::option::Option<::core::primitive::u32>,
    pub logo: ::core::option::Option<::std::string::String>,
    pub language: ::core::option::Option<::std::string::String>,
    pub generator: ::core::option::Option<::std::string::String>,
    pub contact_uri: ::core::option::Option<::std::string::String>,
    pub tags: ::core::option::Option<PackageTagsElementType>,
    pub info: ::core::option::Option<InfoType>,
    pub global: ::core::option::Option<PackageGlobalElementType>,
    pub rounds: ::core::option::Option<PackageRoundsElementType>,
}
impl ::xsd_parser_types::quick_xml::WithSerializer for PackageElementType {
    type Serializer<'x> = quick_xml_serialize::PackageElementTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: ::core::option::Option<&'ser ::core::primitive::str>,
        is_root: ::core::primitive::bool,
    ) -> ::core::result::Result<Self::Serializer<'ser>, ::xsd_parser_types::quick_xml::Error> {
        Ok(quick_xml_serialize::PackageElementTypeSerializer {
            value: self,
            state: ::std::boxed::Box::new(
                quick_xml_serialize::PackageElementTypeSerializerState::Init__,
            ),
            name: name.unwrap_or("package"),
            is_root,
        })
    }
}
impl ::xsd_parser_types::quick_xml::WithDeserializer for PackageElementType {
    type Deserializer = quick_xml_deserialize::PackageElementTypeDeserializer;
}
#[derive(Debug)]
pub struct ParameterType {
    pub name: ::core::option::Option<::std::string::String>,
    pub type_: ::core::option::Option<::std::string::String>,
    pub text_before: ::core::option::Option<::xsd_parser_types::xml::Text>,
    pub item: ::std::vec::Vec<::xsd_parser_types::xml::Mixed<ParameterTypeItemElementType>>,
    pub param: ::std::vec::Vec<::xsd_parser_types::xml::Mixed<ParameterType>>,
    pub number_set:
        ::core::option::Option<::xsd_parser_types::xml::Mixed<ParameterTypeNumberSetElementType>>,
}
impl ::xsd_parser_types::quick_xml::WithSerializer for ParameterType {
    type Serializer<'x> = quick_xml_serialize::ParameterTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: ::core::option::Option<&'ser ::core::primitive::str>,
        is_root: ::core::primitive::bool,
    ) -> ::core::result::Result<Self::Serializer<'ser>, ::xsd_parser_types::quick_xml::Error> {
        Ok(quick_xml_serialize::ParameterTypeSerializer {
            value: self,
            state: ::std::boxed::Box::new(
                quick_xml_serialize::ParameterTypeSerializerState::Init__,
            ),
            name: name.unwrap_or("parameterType"),
            is_root,
        })
    }
}
impl ::xsd_parser_types::quick_xml::WithDeserializer for ParameterType {
    type Deserializer = quick_xml_deserialize::ParameterTypeDeserializer;
}
#[derive(Debug)]
pub struct ParametersType {
    pub param: ::std::vec::Vec<ParameterType>,
}
impl ::xsd_parser_types::quick_xml::WithSerializer for ParametersType {
    type Serializer<'x> = quick_xml_serialize::ParametersTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: ::core::option::Option<&'ser ::core::primitive::str>,
        is_root: ::core::primitive::bool,
    ) -> ::core::result::Result<Self::Serializer<'ser>, ::xsd_parser_types::quick_xml::Error> {
        Ok(quick_xml_serialize::ParametersTypeSerializer {
            value: self,
            state: ::std::boxed::Box::new(
                quick_xml_serialize::ParametersTypeSerializerState::Init__,
            ),
            name: name.unwrap_or("parametersType"),
            is_root,
        })
    }
}
impl ::xsd_parser_types::quick_xml::WithDeserializer for ParametersType {
    type Deserializer = quick_xml_deserialize::ParametersTypeDeserializer;
}
#[derive(Debug)]
pub struct InfoTypeAuthorsElementType {
    pub author: ::std::vec::Vec<::std::string::String>,
}
impl ::xsd_parser_types::quick_xml::WithSerializer for InfoTypeAuthorsElementType {
    type Serializer<'x> = quick_xml_serialize::InfoTypeAuthorsElementTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: ::core::option::Option<&'ser ::core::primitive::str>,
        is_root: ::core::primitive::bool,
    ) -> ::core::result::Result<Self::Serializer<'ser>, ::xsd_parser_types::quick_xml::Error> {
        Ok(quick_xml_serialize::InfoTypeAuthorsElementTypeSerializer {
            value: self,
            state: ::std::boxed::Box::new(
                quick_xml_serialize::InfoTypeAuthorsElementTypeSerializerState::Init__,
            ),
            name: name.unwrap_or("InfoTypeAuthors"),
            is_root,
        })
    }
}
impl ::xsd_parser_types::quick_xml::WithDeserializer for InfoTypeAuthorsElementType {
    type Deserializer = quick_xml_deserialize::InfoTypeAuthorsElementTypeDeserializer;
}
#[derive(Debug)]
pub struct InfoTypeSourcesElementType {
    pub source: ::std::vec::Vec<::std::string::String>,
}
impl ::xsd_parser_types::quick_xml::WithSerializer for InfoTypeSourcesElementType {
    type Serializer<'x> = quick_xml_serialize::InfoTypeSourcesElementTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: ::core::option::Option<&'ser ::core::primitive::str>,
        is_root: ::core::primitive::bool,
    ) -> ::core::result::Result<Self::Serializer<'ser>, ::xsd_parser_types::quick_xml::Error> {
        Ok(quick_xml_serialize::InfoTypeSourcesElementTypeSerializer {
            value: self,
            state: ::std::boxed::Box::new(
                quick_xml_serialize::InfoTypeSourcesElementTypeSerializerState::Init__,
            ),
            name: name.unwrap_or("InfoTypeSources"),
            is_root,
        })
    }
}
impl ::xsd_parser_types::quick_xml::WithDeserializer for InfoTypeSourcesElementType {
    type Deserializer = quick_xml_deserialize::InfoTypeSourcesElementTypeDeserializer;
}
#[derive(Debug)]
pub struct PackageTagsElementType {
    pub tag: ::std::vec::Vec<::std::string::String>,
}
impl ::xsd_parser_types::quick_xml::WithSerializer for PackageTagsElementType {
    type Serializer<'x> = quick_xml_serialize::PackageTagsElementTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: ::core::option::Option<&'ser ::core::primitive::str>,
        is_root: ::core::primitive::bool,
    ) -> ::core::result::Result<Self::Serializer<'ser>, ::xsd_parser_types::quick_xml::Error> {
        Ok(quick_xml_serialize::PackageTagsElementTypeSerializer {
            value: self,
            state: ::std::boxed::Box::new(
                quick_xml_serialize::PackageTagsElementTypeSerializerState::Init__,
            ),
            name: name.unwrap_or("PackageTags"),
            is_root,
        })
    }
}
impl ::xsd_parser_types::quick_xml::WithDeserializer for PackageTagsElementType {
    type Deserializer = quick_xml_deserialize::PackageTagsElementTypeDeserializer;
}
#[derive(Debug)]
pub struct PackageGlobalElementType {
    pub authors: PackageGlobalAuthorsElementType,
    pub sources: PackageGlobalSourcesElementType,
}
impl ::xsd_parser_types::quick_xml::WithSerializer for PackageGlobalElementType {
    type Serializer<'x> = quick_xml_serialize::PackageGlobalElementTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: ::core::option::Option<&'ser ::core::primitive::str>,
        is_root: ::core::primitive::bool,
    ) -> ::core::result::Result<Self::Serializer<'ser>, ::xsd_parser_types::quick_xml::Error> {
        Ok(quick_xml_serialize::PackageGlobalElementTypeSerializer {
            value: self,
            state: ::std::boxed::Box::new(
                quick_xml_serialize::PackageGlobalElementTypeSerializerState::Init__,
            ),
            name: name.unwrap_or("PackageGlobal"),
            is_root,
        })
    }
}
impl ::xsd_parser_types::quick_xml::WithDeserializer for PackageGlobalElementType {
    type Deserializer = quick_xml_deserialize::PackageGlobalElementTypeDeserializer;
}
#[derive(Debug)]
pub struct PackageRoundsElementType {
    pub round: ::std::vec::Vec<PackageRoundsRoundElementType>,
}
impl ::xsd_parser_types::quick_xml::WithSerializer for PackageRoundsElementType {
    type Serializer<'x> = quick_xml_serialize::PackageRoundsElementTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: ::core::option::Option<&'ser ::core::primitive::str>,
        is_root: ::core::primitive::bool,
    ) -> ::core::result::Result<Self::Serializer<'ser>, ::xsd_parser_types::quick_xml::Error> {
        Ok(quick_xml_serialize::PackageRoundsElementTypeSerializer {
            value: self,
            state: ::std::boxed::Box::new(
                quick_xml_serialize::PackageRoundsElementTypeSerializerState::Init__,
            ),
            name: name.unwrap_or("PackageRounds"),
            is_root,
        })
    }
}
impl ::xsd_parser_types::quick_xml::WithDeserializer for PackageRoundsElementType {
    type Deserializer = quick_xml_deserialize::PackageRoundsElementTypeDeserializer;
}
#[derive(Debug)]
pub struct ParameterTypeItemElementType {
    pub type_: ::core::option::Option<::std::string::String>,
    pub is_ref: ::core::option::Option<::std::string::String>,
    pub placement: ::core::option::Option<::std::string::String>,
    pub duration: ::core::option::Option<::std::string::String>,
    pub wait_for_finish: ::core::option::Option<::std::string::String>,
    pub content: ::std::string::String,
}
impl ::xsd_parser_types::quick_xml::WithSerializer for ParameterTypeItemElementType {
    type Serializer<'x> = quick_xml_serialize::ParameterTypeItemElementTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: ::core::option::Option<&'ser ::core::primitive::str>,
        is_root: ::core::primitive::bool,
    ) -> ::core::result::Result<Self::Serializer<'ser>, ::xsd_parser_types::quick_xml::Error> {
        Ok(
            quick_xml_serialize::ParameterTypeItemElementTypeSerializer {
                value: self,
                state: ::std::boxed::Box::new(
                    quick_xml_serialize::ParameterTypeItemElementTypeSerializerState::Init__,
                ),
                name: name.unwrap_or("ParameterTypeItem"),
                is_root,
            },
        )
    }
}
impl ::xsd_parser_types::quick_xml::WithDeserializer for ParameterTypeItemElementType {
    type Deserializer = quick_xml_deserialize::ParameterTypeItemElementTypeDeserializer;
}
#[derive(Debug)]
pub struct ParameterTypeNumberSetElementType {
    pub minimum: ::core::option::Option<::core::primitive::i32>,
    pub maximum: ::core::option::Option<::core::primitive::i32>,
    pub step: ::core::option::Option<::core::primitive::i32>,
    pub content: ::std::string::String,
}
impl ::xsd_parser_types::quick_xml::WithSerializer for ParameterTypeNumberSetElementType {
    type Serializer<'x> = quick_xml_serialize::ParameterTypeNumberSetElementTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: ::core::option::Option<&'ser ::core::primitive::str>,
        is_root: ::core::primitive::bool,
    ) -> ::core::result::Result<Self::Serializer<'ser>, ::xsd_parser_types::quick_xml::Error> {
        Ok(
            quick_xml_serialize::ParameterTypeNumberSetElementTypeSerializer {
                value: self,
                state: ::std::boxed::Box::new(
                    quick_xml_serialize::ParameterTypeNumberSetElementTypeSerializerState::Init__,
                ),
                name: name.unwrap_or("ParameterTypeNumberSet"),
                is_root,
            },
        )
    }
}
impl ::xsd_parser_types::quick_xml::WithDeserializer for ParameterTypeNumberSetElementType {
    type Deserializer = quick_xml_deserialize::ParameterTypeNumberSetElementTypeDeserializer;
}
#[derive(Debug)]
pub struct PackageGlobalAuthorsElementType {
    pub id: ::std::string::String,
    pub name: ::std::string::String,
    pub second_name: ::std::string::String,
    pub surname: ::std::string::String,
    pub country: ::std::string::String,
    pub city: ::std::string::String,
}
impl ::xsd_parser_types::quick_xml::WithSerializer for PackageGlobalAuthorsElementType {
    type Serializer<'x> = quick_xml_serialize::PackageGlobalAuthorsElementTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: ::core::option::Option<&'ser ::core::primitive::str>,
        is_root: ::core::primitive::bool,
    ) -> ::core::result::Result<Self::Serializer<'ser>, ::xsd_parser_types::quick_xml::Error> {
        Ok(
            quick_xml_serialize::PackageGlobalAuthorsElementTypeSerializer {
                value: self,
                state: ::std::boxed::Box::new(
                    quick_xml_serialize::PackageGlobalAuthorsElementTypeSerializerState::Init__,
                ),
                name: name.unwrap_or("PackageGlobalAuthors"),
                is_root,
            },
        )
    }
}
impl ::xsd_parser_types::quick_xml::WithDeserializer for PackageGlobalAuthorsElementType {
    type Deserializer = quick_xml_deserialize::PackageGlobalAuthorsElementTypeDeserializer;
}
#[derive(Debug)]
pub struct PackageGlobalSourcesElementType {
    pub id: ::std::string::String,
    pub author: ::std::string::String,
    pub title: ::std::string::String,
    pub year: ::std::string::String,
    pub publish: ::std::string::String,
    pub city: ::std::string::String,
}
impl ::xsd_parser_types::quick_xml::WithSerializer for PackageGlobalSourcesElementType {
    type Serializer<'x> = quick_xml_serialize::PackageGlobalSourcesElementTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: ::core::option::Option<&'ser ::core::primitive::str>,
        is_root: ::core::primitive::bool,
    ) -> ::core::result::Result<Self::Serializer<'ser>, ::xsd_parser_types::quick_xml::Error> {
        Ok(
            quick_xml_serialize::PackageGlobalSourcesElementTypeSerializer {
                value: self,
                state: ::std::boxed::Box::new(
                    quick_xml_serialize::PackageGlobalSourcesElementTypeSerializerState::Init__,
                ),
                name: name.unwrap_or("PackageGlobalSources"),
                is_root,
            },
        )
    }
}
impl ::xsd_parser_types::quick_xml::WithDeserializer for PackageGlobalSourcesElementType {
    type Deserializer = quick_xml_deserialize::PackageGlobalSourcesElementTypeDeserializer;
}
#[derive(Debug)]
pub struct PackageRoundsRoundElementType {
    pub name: ::std::string::String,
    pub type_: ::core::option::Option<::std::string::String>,
    pub info: ::core::option::Option<InfoType>,
    pub themes: ::core::option::Option<PackageRoundsRoundThemesElementType>,
}
impl ::xsd_parser_types::quick_xml::WithSerializer for PackageRoundsRoundElementType {
    type Serializer<'x> = quick_xml_serialize::PackageRoundsRoundElementTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: ::core::option::Option<&'ser ::core::primitive::str>,
        is_root: ::core::primitive::bool,
    ) -> ::core::result::Result<Self::Serializer<'ser>, ::xsd_parser_types::quick_xml::Error> {
        Ok(
            quick_xml_serialize::PackageRoundsRoundElementTypeSerializer {
                value: self,
                state: ::std::boxed::Box::new(
                    quick_xml_serialize::PackageRoundsRoundElementTypeSerializerState::Init__,
                ),
                name: name.unwrap_or("PackageRoundsRound"),
                is_root,
            },
        )
    }
}
impl ::xsd_parser_types::quick_xml::WithDeserializer for PackageRoundsRoundElementType {
    type Deserializer = quick_xml_deserialize::PackageRoundsRoundElementTypeDeserializer;
}
#[derive(Debug)]
pub struct PackageRoundsRoundThemesElementType {
    pub theme: ::std::vec::Vec<PackageRoundsRoundThemesThemeElementType>,
}
impl ::xsd_parser_types::quick_xml::WithSerializer for PackageRoundsRoundThemesElementType {
    type Serializer<'x> = quick_xml_serialize::PackageRoundsRoundThemesElementTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: ::core::option::Option<&'ser ::core::primitive::str>,
        is_root: ::core::primitive::bool,
    ) -> ::core::result::Result<Self::Serializer<'ser>, ::xsd_parser_types::quick_xml::Error> {
        Ok(
            quick_xml_serialize::PackageRoundsRoundThemesElementTypeSerializer {
                value: self,
                state: ::std::boxed::Box::new(
                    quick_xml_serialize::PackageRoundsRoundThemesElementTypeSerializerState::Init__,
                ),
                name: name.unwrap_or("PackageRoundsRoundThemes"),
                is_root,
            },
        )
    }
}
impl ::xsd_parser_types::quick_xml::WithDeserializer for PackageRoundsRoundThemesElementType {
    type Deserializer = quick_xml_deserialize::PackageRoundsRoundThemesElementTypeDeserializer;
}
#[derive(Debug)]
pub struct PackageRoundsRoundThemesThemeElementType {
    pub name: ::std::string::String,
    pub info: ::core::option::Option<InfoType>,
    pub questions: ::core::option::Option<PackageRoundsRoundThemesThemeQuestionsElementType>,
}
impl ::xsd_parser_types::quick_xml::WithSerializer for PackageRoundsRoundThemesThemeElementType {
    type Serializer<'x> =
        quick_xml_serialize::PackageRoundsRoundThemesThemeElementTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: ::core::option::Option<&'ser ::core::primitive::str>,
        is_root: ::core::primitive::bool,
    ) -> ::core::result::Result<Self::Serializer<'ser>, ::xsd_parser_types::quick_xml::Error> {
        Ok (quick_xml_serialize :: PackageRoundsRoundThemesThemeElementTypeSerializer { value : self , state : :: std :: boxed :: Box :: new (quick_xml_serialize :: PackageRoundsRoundThemesThemeElementTypeSerializerState :: Init__) , name : name . unwrap_or ("PackageRoundsRoundThemesTheme") , is_root , })
    }
}
impl ::xsd_parser_types::quick_xml::WithDeserializer for PackageRoundsRoundThemesThemeElementType {
    type Deserializer = quick_xml_deserialize::PackageRoundsRoundThemesThemeElementTypeDeserializer;
}
#[derive(Debug)]
pub struct PackageRoundsRoundThemesThemeQuestionsElementType {
    pub question: ::std::vec::Vec<PackageRoundsRoundThemesThemeQuestionsQuestionElementType>,
}
impl ::xsd_parser_types::quick_xml::WithSerializer
    for PackageRoundsRoundThemesThemeQuestionsElementType
{
    type Serializer<'x> =
        quick_xml_serialize::PackageRoundsRoundThemesThemeQuestionsElementTypeSerializer<'x>;
    fn serializer<'ser>(
        &'ser self,
        name: ::core::option::Option<&'ser ::core::primitive::str>,
        is_root: ::core::primitive::bool,
    ) -> ::core::result::Result<Self::Serializer<'ser>, ::xsd_parser_types::quick_xml::Error> {
        Ok (quick_xml_serialize :: PackageRoundsRoundThemesThemeQuestionsElementTypeSerializer { value : self , state : :: std :: boxed :: Box :: new (quick_xml_serialize :: PackageRoundsRoundThemesThemeQuestionsElementTypeSerializerState :: Init__) , name : name . unwrap_or ("PackageRoundsRoundThemesThemeQuestions") , is_root , })
    }
}
impl ::xsd_parser_types::quick_xml::WithDeserializer
    for PackageRoundsRoundThemesThemeQuestionsElementType
{
    type Deserializer =
        quick_xml_deserialize::PackageRoundsRoundThemesThemeQuestionsElementTypeDeserializer;
}
#[derive(Debug)]
pub struct PackageRoundsRoundThemesThemeQuestionsQuestionElementType {
    pub price: ::core::primitive::i32,
    pub type_: ::core::option::Option<::std::string::String>,
    pub info: ::core::option::Option<InfoType>,
    pub type_name:
        ::core::option::Option<PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameElementType>,
    pub scenario:
        ::core::option::Option<PackageRoundsRoundThemesThemeQuestionsQuestionScenarioElementType>,
    pub script:
        ::core::option::Option<PackageRoundsRoundThemesThemeQuestionsQuestionScriptElementType>,
    pub params: ::core::option::Option<ParametersType>,
    pub right: PackageRoundsRoundThemesThemeQuestionsQuestionRightElementType,
    pub wrong:
        ::core::option::Option<PackageRoundsRoundThemesThemeQuestionsQuestionRightElementType>,
}
impl ::xsd_parser_types::quick_xml::WithSerializer
    for PackageRoundsRoundThemesThemeQuestionsQuestionElementType
{
    type Serializer<'x> =
        quick_xml_serialize::PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeSerializer<
            'x,
        >;
    fn serializer<'ser>(
        &'ser self,
        name: ::core::option::Option<&'ser ::core::primitive::str>,
        is_root: ::core::primitive::bool,
    ) -> ::core::result::Result<Self::Serializer<'ser>, ::xsd_parser_types::quick_xml::Error> {
        Ok (quick_xml_serialize :: PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeSerializer { value : self , state : :: std :: boxed :: Box :: new (quick_xml_serialize :: PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeSerializerState :: Init__) , name : name . unwrap_or ("PackageRoundsRoundThemesThemeQuestionsQuestion") , is_root , })
    }
}
impl ::xsd_parser_types::quick_xml::WithDeserializer
    for PackageRoundsRoundThemesThemeQuestionsQuestionElementType
{
    type Deserializer = quick_xml_deserialize :: PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeDeserializer ;
}
#[derive(Debug)]
pub struct PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameElementType {
    pub name: ::std::string::String,
    pub param:
        ::std::vec::Vec<PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameParamElementType>,
}
impl ::xsd_parser_types::quick_xml::WithSerializer
    for PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameElementType
{
    type Serializer < 'x > = quick_xml_serialize :: PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameElementTypeSerializer < 'x > ;
    fn serializer<'ser>(
        &'ser self,
        name: ::core::option::Option<&'ser ::core::primitive::str>,
        is_root: ::core::primitive::bool,
    ) -> ::core::result::Result<Self::Serializer<'ser>, ::xsd_parser_types::quick_xml::Error> {
        Ok (quick_xml_serialize :: PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameElementTypeSerializer { value : self , state : :: std :: boxed :: Box :: new (quick_xml_serialize :: PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameElementTypeSerializerState :: Init__) , name : name . unwrap_or ("PackageRoundsRoundThemesThemeQuestionsQuestionTypeName") , is_root , })
    }
}
impl ::xsd_parser_types::quick_xml::WithDeserializer
    for PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameElementType
{
    type Deserializer = quick_xml_deserialize :: PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameElementTypeDeserializer ;
}
#[derive(Debug)]
pub struct PackageRoundsRoundThemesThemeQuestionsQuestionScenarioElementType {
    pub atom:
        ::std::vec::Vec<PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomElementType>,
}
impl ::xsd_parser_types::quick_xml::WithSerializer
    for PackageRoundsRoundThemesThemeQuestionsQuestionScenarioElementType
{
    type Serializer < 'x > = quick_xml_serialize :: PackageRoundsRoundThemesThemeQuestionsQuestionScenarioElementTypeSerializer < 'x > ;
    fn serializer<'ser>(
        &'ser self,
        name: ::core::option::Option<&'ser ::core::primitive::str>,
        is_root: ::core::primitive::bool,
    ) -> ::core::result::Result<Self::Serializer<'ser>, ::xsd_parser_types::quick_xml::Error> {
        Ok (quick_xml_serialize :: PackageRoundsRoundThemesThemeQuestionsQuestionScenarioElementTypeSerializer { value : self , state : :: std :: boxed :: Box :: new (quick_xml_serialize :: PackageRoundsRoundThemesThemeQuestionsQuestionScenarioElementTypeSerializerState :: Init__) , name : name . unwrap_or ("PackageRoundsRoundThemesThemeQuestionsQuestionScenario") , is_root , })
    }
}
impl ::xsd_parser_types::quick_xml::WithDeserializer
    for PackageRoundsRoundThemesThemeQuestionsQuestionScenarioElementType
{
    type Deserializer = quick_xml_deserialize :: PackageRoundsRoundThemesThemeQuestionsQuestionScenarioElementTypeDeserializer ;
}
#[derive(Debug)]
pub struct PackageRoundsRoundThemesThemeQuestionsQuestionScriptElementType {
    pub step: ::std::vec::Vec<ParametersType>,
}
impl ::xsd_parser_types::quick_xml::WithSerializer
    for PackageRoundsRoundThemesThemeQuestionsQuestionScriptElementType
{
    type Serializer < 'x > = quick_xml_serialize :: PackageRoundsRoundThemesThemeQuestionsQuestionScriptElementTypeSerializer < 'x > ;
    fn serializer<'ser>(
        &'ser self,
        name: ::core::option::Option<&'ser ::core::primitive::str>,
        is_root: ::core::primitive::bool,
    ) -> ::core::result::Result<Self::Serializer<'ser>, ::xsd_parser_types::quick_xml::Error> {
        Ok (quick_xml_serialize :: PackageRoundsRoundThemesThemeQuestionsQuestionScriptElementTypeSerializer { value : self , state : :: std :: boxed :: Box :: new (quick_xml_serialize :: PackageRoundsRoundThemesThemeQuestionsQuestionScriptElementTypeSerializerState :: Init__) , name : name . unwrap_or ("PackageRoundsRoundThemesThemeQuestionsQuestionScript") , is_root , })
    }
}
impl ::xsd_parser_types::quick_xml::WithDeserializer
    for PackageRoundsRoundThemesThemeQuestionsQuestionScriptElementType
{
    type Deserializer = quick_xml_deserialize :: PackageRoundsRoundThemesThemeQuestionsQuestionScriptElementTypeDeserializer ;
}
#[derive(Debug)]
pub struct PackageRoundsRoundThemesThemeQuestionsQuestionRightElementType {
    pub answer: ::std::vec::Vec<::std::string::String>,
}
impl ::xsd_parser_types::quick_xml::WithSerializer
    for PackageRoundsRoundThemesThemeQuestionsQuestionRightElementType
{
    type Serializer < 'x > = quick_xml_serialize :: PackageRoundsRoundThemesThemeQuestionsQuestionRightElementTypeSerializer < 'x > ;
    fn serializer<'ser>(
        &'ser self,
        name: ::core::option::Option<&'ser ::core::primitive::str>,
        is_root: ::core::primitive::bool,
    ) -> ::core::result::Result<Self::Serializer<'ser>, ::xsd_parser_types::quick_xml::Error> {
        Ok (quick_xml_serialize :: PackageRoundsRoundThemesThemeQuestionsQuestionRightElementTypeSerializer { value : self , state : :: std :: boxed :: Box :: new (quick_xml_serialize :: PackageRoundsRoundThemesThemeQuestionsQuestionRightElementTypeSerializerState :: Init__) , name : name . unwrap_or ("PackageRoundsRoundThemesThemeQuestionsQuestionRight") , is_root , })
    }
}
impl ::xsd_parser_types::quick_xml::WithDeserializer
    for PackageRoundsRoundThemesThemeQuestionsQuestionRightElementType
{
    type Deserializer = quick_xml_deserialize :: PackageRoundsRoundThemesThemeQuestionsQuestionRightElementTypeDeserializer ;
}
#[derive(Debug)]
pub struct PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameParamElementType {
    pub name: ::std::string::String,
    pub content: ::std::string::String,
}
impl ::xsd_parser_types::quick_xml::WithSerializer
    for PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameParamElementType
{
    type Serializer < 'x > = quick_xml_serialize :: PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameParamElementTypeSerializer < 'x > ;
    fn serializer<'ser>(
        &'ser self,
        name: ::core::option::Option<&'ser ::core::primitive::str>,
        is_root: ::core::primitive::bool,
    ) -> ::core::result::Result<Self::Serializer<'ser>, ::xsd_parser_types::quick_xml::Error> {
        Ok (quick_xml_serialize :: PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameParamElementTypeSerializer { value : self , state : :: std :: boxed :: Box :: new (quick_xml_serialize :: PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameParamElementTypeSerializerState :: Init__) , name : name . unwrap_or ("PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameParam") , is_root , })
    }
}
impl ::xsd_parser_types::quick_xml::WithDeserializer
    for PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameParamElementType
{
    type Deserializer = quick_xml_deserialize :: PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameParamElementTypeDeserializer ;
}
#[derive(Debug)]
pub struct PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomElementType {
    pub type_:
        ::core::option::Option<PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomType>,
    pub time: ::core::option::Option<::core::primitive::f64>,
    pub content: ::std::string::String,
}
impl ::xsd_parser_types::quick_xml::WithSerializer
    for PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomElementType
{
    type Serializer < 'x > = quick_xml_serialize :: PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomElementTypeSerializer < 'x > ;
    fn serializer<'ser>(
        &'ser self,
        name: ::core::option::Option<&'ser ::core::primitive::str>,
        is_root: ::core::primitive::bool,
    ) -> ::core::result::Result<Self::Serializer<'ser>, ::xsd_parser_types::quick_xml::Error> {
        Ok (quick_xml_serialize :: PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomElementTypeSerializer { value : self , state : :: std :: boxed :: Box :: new (quick_xml_serialize :: PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomElementTypeSerializerState :: Init__) , name : name . unwrap_or ("PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtom") , is_root , })
    }
}
impl ::xsd_parser_types::quick_xml::WithDeserializer
    for PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomElementType
{
    type Deserializer = quick_xml_deserialize :: PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomElementTypeDeserializer ;
}
#[derive(Debug)]
pub enum PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomType {
    Text,
    Say,
    Image,
    Voice,
    Video,
    Marker,
}
impl ::xsd_parser_types::quick_xml::SerializeBytes
    for PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomType
{
    fn serialize_bytes(
        &self,
        helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
    ) -> ::core::result::Result<
        ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
        ::xsd_parser_types::quick_xml::Error,
    > {
        match self {
            Self::Text => Ok(Some(::std::borrow::Cow::Borrowed("text"))),
            Self::Say => Ok(Some(::std::borrow::Cow::Borrowed("say"))),
            Self::Image => Ok(Some(::std::borrow::Cow::Borrowed("image"))),
            Self::Voice => Ok(Some(::std::borrow::Cow::Borrowed("voice"))),
            Self::Video => Ok(Some(::std::borrow::Cow::Borrowed("video"))),
            Self::Marker => Ok(Some(::std::borrow::Cow::Borrowed("marker"))),
        }
    }
}
impl ::xsd_parser_types::quick_xml::DeserializeBytes
    for PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomType
{
    fn deserialize_bytes(
        helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        bytes: &[::core::primitive::u8],
    ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
        match bytes {
            b"text" => Ok(Self::Text),
            b"say" => Ok(Self::Say),
            b"image" => Ok(Self::Image),
            b"voice" => Ok(Self::Voice),
            b"video" => Ok(Self::Video),
            b"marker" => Ok(Self::Marker),
            x => Err(::xsd_parser_types::quick_xml::Error::from(
                ::xsd_parser_types::quick_xml::ErrorKind::UnknownOrInvalidValue(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(x),
                ),
            )),
        }
    }
}
pub mod quick_xml_deserialize {
    use xsd_parser_types::quick_xml::Deserializer as _;
    #[derive(Debug)]
    pub struct InfoTypeDeserializer {
        authors: ::core::option::Option<super::InfoTypeAuthorsElementType>,
        sources: ::core::option::Option<super::InfoTypeSourcesElementType>,
        comments: ::core::option::Option<::std::string::String>,
        showman_comments: ::core::option::Option<::std::string::String>,
        extension: ::core::option::Option<::std::string::String>,
        state__: ::std::boxed::Box<InfoTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum InfoTypeDeserializerState {
        Init__ , Next__ , Authors (< super :: InfoTypeAuthorsElementType as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer) , Sources (< super :: InfoTypeSourcesElementType as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer) , Comments (< :: std :: string :: String as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer) , ShowmanComments (< :: std :: string :: String as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer) , Extension (< :: std :: string :: String as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer) , Unknown__ , }
    impl InfoTypeDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
            fallback: &mut ::core::option::Option<InfoTypeDeserializerState>,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            if let ::xsd_parser_types::quick_xml::Event::Start(x)
            | ::xsd_parser_types::quick_xml::Event::Empty(x) = &event
            {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_UNNAMED_2),
                    Some(b"authors")
                ) {
                    let output = < super :: InfoTypeAuthorsElementType as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: init (helper , event) ? ;
                    return self.handle_authors(helper, output, &mut *fallback);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_UNNAMED_2),
                    Some(b"sources")
                ) {
                    let output = < super :: InfoTypeSourcesElementType as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: init (helper , event) ? ;
                    return self.handle_sources(helper, output, &mut *fallback);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_UNNAMED_2),
                    Some(b"comments")
                ) {
                    let output = < :: std :: string :: String as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: init (helper , event) ? ;
                    return self.handle_comments(helper, output, &mut *fallback);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_UNNAMED_2),
                    Some(b"showmanComments")
                ) {
                    let output = < :: std :: string :: String as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: init (helper , event) ? ;
                    return self.handle_showman_comments(helper, output, &mut *fallback);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_UNNAMED_2),
                    Some(b"extension")
                ) {
                    let output = < :: std :: string :: String as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: init (helper , event) ? ;
                    return self.handle_extension(helper, output, &mut *fallback);
                }
            }
            *self.state__ = fallback.take().unwrap_or(InfoTypeDeserializerState::Init__);
            Ok(::xsd_parser_types::quick_xml::ElementHandlerOutput::return_to_parent(event, false))
        }
        fn from_bytes_start(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            bytes_start: &::xsd_parser_types::quick_xml::BytesStart<'_>,
        ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                authors: None,
                sources: None,
                comments: None,
                showman_comments: None,
                extension: None,
                state__: ::std::boxed::Box::new(InfoTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            state: InfoTypeDeserializerState,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            use InfoTypeDeserializerState as S;
            match state {
                S::Authors(deserializer) => self.store_authors(deserializer.finish(helper)?)?,
                S::Sources(deserializer) => self.store_sources(deserializer.finish(helper)?)?,
                S::Comments(deserializer) => self.store_comments(deserializer.finish(helper)?)?,
                S::ShowmanComments(deserializer) => {
                    self.store_showman_comments(deserializer.finish(helper)?)?
                }
                S::Extension(deserializer) => self.store_extension(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_authors(
            &mut self,
            value: super::InfoTypeAuthorsElementType,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            if self.authors.is_some() {
                Err(::xsd_parser_types::quick_xml::ErrorKind::DuplicateElement(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(b"authors"),
                ))?;
            }
            self.authors = Some(value);
            Ok(())
        }
        fn store_sources(
            &mut self,
            value: super::InfoTypeSourcesElementType,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            if self.sources.is_some() {
                Err(::xsd_parser_types::quick_xml::ErrorKind::DuplicateElement(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(b"sources"),
                ))?;
            }
            self.sources = Some(value);
            Ok(())
        }
        fn store_comments(
            &mut self,
            value: ::std::string::String,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            if self.comments.is_some() {
                Err(::xsd_parser_types::quick_xml::ErrorKind::DuplicateElement(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(b"comments"),
                ))?;
            }
            self.comments = Some(value);
            Ok(())
        }
        fn store_showman_comments(
            &mut self,
            value: ::std::string::String,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            if self.showman_comments.is_some() {
                Err(::xsd_parser_types::quick_xml::ErrorKind::DuplicateElement(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(b"showmanComments"),
                ))?;
            }
            self.showman_comments = Some(value);
            Ok(())
        }
        fn store_extension(
            &mut self,
            value: ::std::string::String,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            if self.extension.is_some() {
                Err(::xsd_parser_types::quick_xml::ErrorKind::DuplicateElement(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(b"extension"),
                ))?;
            }
            self.extension = Some(value);
            Ok(())
        }
        fn handle_authors<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<
                'de,
                super::InfoTypeAuthorsElementType,
            >,
            fallback: &mut ::core::option::Option<InfoTypeDeserializerState>,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use InfoTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Next__;
                return Ok(
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput::return_to_root(
                        event, allow_any,
                    ),
                );
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_authors(data)?;
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Authors(deserializer));
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
        fn handle_sources<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<
                'de,
                super::InfoTypeSourcesElementType,
            >,
            fallback: &mut ::core::option::Option<InfoTypeDeserializerState>,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use InfoTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Next__;
                return Ok(
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput::return_to_root(
                        event, allow_any,
                    ),
                );
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_sources(data)?;
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Sources(deserializer));
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
        fn handle_comments<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<'de, ::std::string::String>,
            fallback: &mut ::core::option::Option<InfoTypeDeserializerState>,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use InfoTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Next__;
                return Ok(
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput::return_to_root(
                        event, allow_any,
                    ),
                );
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_comments(data)?;
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Comments(deserializer));
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
        fn handle_showman_comments<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<'de, ::std::string::String>,
            fallback: &mut ::core::option::Option<InfoTypeDeserializerState>,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use InfoTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Next__;
                return Ok(
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput::return_to_root(
                        event, allow_any,
                    ),
                );
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_showman_comments(data)?;
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::ShowmanComments(deserializer));
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
        fn handle_extension<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<'de, ::std::string::String>,
            fallback: &mut ::core::option::Option<InfoTypeDeserializerState>,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use InfoTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Next__;
                return Ok(
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput::return_to_root(
                        event, allow_any,
                    ),
                );
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_extension(data)?;
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Extension(deserializer));
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
    }
    impl<'de> ::xsd_parser_types::quick_xml::Deserializer<'de, super::InfoType>
        for InfoTypeDeserializer
    {
        fn init(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<'de, super::InfoType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<'de, super::InfoType> {
            use InfoTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = ::core::mem::replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Authors(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_authors(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                ..
                            } => event,
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (S::Sources(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_sources(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                ..
                            } => event,
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (S::Comments(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_comments(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                ..
                            } => event,
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (S::ShowmanComments(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_showman_comments(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                ..
                            } => event,
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (S::Extension(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_extension(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                ..
                            } => event,
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (_, ::xsd_parser_types::quick_xml::Event::End(_)) => {
                        return Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                            artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(
                                self.finish(helper)?,
                            ),
                            event: ::xsd_parser_types::quick_xml::DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        match self.find_suitable(helper, event, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                ..
                            } => event,
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        ) -> ::core::result::Result<super::InfoType, ::xsd_parser_types::quick_xml::Error> {
            let state =
                ::core::mem::replace(&mut *self.state__, InfoTypeDeserializerState::Unknown__);
            self.finish_state(helper, state)?;
            Ok(super::InfoType {
                authors: self.authors,
                sources: self.sources,
                comments: self.comments,
                showman_comments: self.showman_comments,
                extension: self.extension,
            })
        }
    }
    #[derive(Debug)]
    pub struct PackageElementTypeDeserializer {
        id: ::core::option::Option<::std::string::String>,
        name: ::std::string::String,
        version: ::core::primitive::f64,
        restriction: ::core::option::Option<::std::string::String>,
        date: ::core::option::Option<::std::string::String>,
        publisher: ::core::option::Option<::std::string::String>,
        difficulty: ::core::option::Option<::core::primitive::u32>,
        logo: ::core::option::Option<::std::string::String>,
        language: ::core::option::Option<::std::string::String>,
        generator: ::core::option::Option<::std::string::String>,
        contact_uri: ::core::option::Option<::std::string::String>,
        tags: ::core::option::Option<super::PackageTagsElementType>,
        info: ::core::option::Option<super::InfoType>,
        global: ::core::option::Option<super::PackageGlobalElementType>,
        rounds: ::core::option::Option<super::PackageRoundsElementType>,
        state__: ::std::boxed::Box<PackageElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum PackageElementTypeDeserializerState {
        Init__ , Next__ , Tags (< super :: PackageTagsElementType as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer) , Info (< super :: InfoType as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer) , Global (< super :: PackageGlobalElementType as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer) , Rounds (< super :: PackageRoundsElementType as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer) , Unknown__ , }
    impl PackageElementTypeDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
            fallback: &mut ::core::option::Option<PackageElementTypeDeserializerState>,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            if let ::xsd_parser_types::quick_xml::Event::Start(x)
            | ::xsd_parser_types::quick_xml::Event::Empty(x) = &event
            {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_UNNAMED_2),
                    Some(b"tags")
                ) {
                    let output = < super :: PackageTagsElementType as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: init (helper , event) ? ;
                    return self.handle_tags(helper, output, &mut *fallback);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_UNNAMED_2),
                    Some(b"info")
                ) {
                    let output =
                        <super::InfoType as ::xsd_parser_types::quick_xml::WithDeserializer>::init(
                            helper, event,
                        )?;
                    return self.handle_info(helper, output, &mut *fallback);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_UNNAMED_2),
                    Some(b"global")
                ) {
                    let output = < super :: PackageGlobalElementType as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: init (helper , event) ? ;
                    return self.handle_global(helper, output, &mut *fallback);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_UNNAMED_2),
                    Some(b"rounds")
                ) {
                    let output = < super :: PackageRoundsElementType as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: init (helper , event) ? ;
                    return self.handle_rounds(helper, output, &mut *fallback);
                }
            }
            *self.state__ = fallback
                .take()
                .unwrap_or(PackageElementTypeDeserializerState::Init__);
            Ok(::xsd_parser_types::quick_xml::ElementHandlerOutput::return_to_parent(event, false))
        }
        fn from_bytes_start(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            bytes_start: &::xsd_parser_types::quick_xml::BytesStart<'_>,
        ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
            let mut id: ::core::option::Option<::std::string::String> = None;
            let mut name: ::core::option::Option<::std::string::String> = None;
            let mut version: ::core::option::Option<::core::primitive::f64> = None;
            let mut restriction: ::core::option::Option<::std::string::String> = None;
            let mut date: ::core::option::Option<::std::string::String> = None;
            let mut publisher: ::core::option::Option<::std::string::String> = None;
            let mut difficulty: ::core::option::Option<::core::primitive::u32> = None;
            let mut logo: ::core::option::Option<::std::string::String> = None;
            let mut language: ::core::option::Option<::std::string::String> = None;
            let mut generator: ::core::option::Option<::std::string::String> = None;
            let mut contact_uri: ::core::option::Option<::std::string::String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_UNNAMED_2),
                    Some(b"id")
                ) {
                    helper.read_attrib(&mut id, b"id", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_UNNAMED_2),
                    Some(b"name")
                ) {
                    helper.read_attrib(&mut name, b"name", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_UNNAMED_2),
                    Some(b"version")
                ) {
                    helper.read_attrib(&mut version, b"version", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_UNNAMED_2),
                    Some(b"restriction")
                ) {
                    helper.read_attrib(&mut restriction, b"restriction", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_UNNAMED_2),
                    Some(b"date")
                ) {
                    helper.read_attrib(&mut date, b"date", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_UNNAMED_2),
                    Some(b"publisher")
                ) {
                    helper.read_attrib(&mut publisher, b"publisher", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_UNNAMED_2),
                    Some(b"difficulty")
                ) {
                    helper.read_attrib(&mut difficulty, b"difficulty", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_UNNAMED_2),
                    Some(b"logo")
                ) {
                    helper.read_attrib(&mut logo, b"logo", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_UNNAMED_2),
                    Some(b"language")
                ) {
                    helper.read_attrib(&mut language, b"language", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_UNNAMED_2),
                    Some(b"generator")
                ) {
                    helper.read_attrib(&mut generator, b"generator", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_UNNAMED_2),
                    Some(b"contactUri")
                ) {
                    helper.read_attrib(&mut contact_uri, b"contactUri", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                id: id,
                name: name.ok_or_else(|| {
                    ::xsd_parser_types::quick_xml::ErrorKind::MissingAttribute("name".into())
                })?,
                version: version.ok_or_else(|| {
                    ::xsd_parser_types::quick_xml::ErrorKind::MissingAttribute("version".into())
                })?,
                restriction: restriction,
                date: date,
                publisher: publisher,
                difficulty: difficulty,
                logo: logo,
                language: language,
                generator: generator,
                contact_uri: contact_uri,
                tags: None,
                info: None,
                global: None,
                rounds: None,
                state__: ::std::boxed::Box::new(PackageElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            state: PackageElementTypeDeserializerState,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            use PackageElementTypeDeserializerState as S;
            match state {
                S::Tags(deserializer) => self.store_tags(deserializer.finish(helper)?)?,
                S::Info(deserializer) => self.store_info(deserializer.finish(helper)?)?,
                S::Global(deserializer) => self.store_global(deserializer.finish(helper)?)?,
                S::Rounds(deserializer) => self.store_rounds(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_tags(
            &mut self,
            value: super::PackageTagsElementType,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            if self.tags.is_some() {
                Err(::xsd_parser_types::quick_xml::ErrorKind::DuplicateElement(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(b"tags"),
                ))?;
            }
            self.tags = Some(value);
            Ok(())
        }
        fn store_info(
            &mut self,
            value: super::InfoType,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            if self.info.is_some() {
                Err(::xsd_parser_types::quick_xml::ErrorKind::DuplicateElement(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(b"info"),
                ))?;
            }
            self.info = Some(value);
            Ok(())
        }
        fn store_global(
            &mut self,
            value: super::PackageGlobalElementType,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            if self.global.is_some() {
                Err(::xsd_parser_types::quick_xml::ErrorKind::DuplicateElement(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(b"global"),
                ))?;
            }
            self.global = Some(value);
            Ok(())
        }
        fn store_rounds(
            &mut self,
            value: super::PackageRoundsElementType,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            if self.rounds.is_some() {
                Err(::xsd_parser_types::quick_xml::ErrorKind::DuplicateElement(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(b"rounds"),
                ))?;
            }
            self.rounds = Some(value);
            Ok(())
        }
        fn handle_tags<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<
                'de,
                super::PackageTagsElementType,
            >,
            fallback: &mut ::core::option::Option<PackageElementTypeDeserializerState>,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use PackageElementTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Next__;
                return Ok(
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput::return_to_root(
                        event, allow_any,
                    ),
                );
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_tags(data)?;
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Tags(deserializer));
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
        fn handle_info<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<'de, super::InfoType>,
            fallback: &mut ::core::option::Option<PackageElementTypeDeserializerState>,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use PackageElementTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Next__;
                return Ok(
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput::return_to_root(
                        event, allow_any,
                    ),
                );
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_info(data)?;
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Info(deserializer));
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
        fn handle_global<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<
                'de,
                super::PackageGlobalElementType,
            >,
            fallback: &mut ::core::option::Option<PackageElementTypeDeserializerState>,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use PackageElementTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Next__;
                return Ok(
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput::return_to_root(
                        event, allow_any,
                    ),
                );
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_global(data)?;
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Global(deserializer));
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
        fn handle_rounds<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<
                'de,
                super::PackageRoundsElementType,
            >,
            fallback: &mut ::core::option::Option<PackageElementTypeDeserializerState>,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use PackageElementTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Next__;
                return Ok(
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput::return_to_root(
                        event, allow_any,
                    ),
                );
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_rounds(data)?;
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Rounds(deserializer));
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
    }
    impl<'de> ::xsd_parser_types::quick_xml::Deserializer<'de, super::PackageElementType>
        for PackageElementTypeDeserializer
    {
        fn init(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<'de, super::PackageElementType>
        {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<'de, super::PackageElementType>
        {
            use PackageElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = ::core::mem::replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Tags(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_tags(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                ..
                            } => event,
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (S::Info(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_info(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                ..
                            } => event,
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (S::Global(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_global(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                ..
                            } => event,
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (S::Rounds(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_rounds(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                ..
                            } => event,
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (_, ::xsd_parser_types::quick_xml::Event::End(_)) => {
                        return Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                            artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(
                                self.finish(helper)?,
                            ),
                            event: ::xsd_parser_types::quick_xml::DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        match self.find_suitable(helper, event, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                ..
                            } => event,
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        ) -> ::core::result::Result<super::PackageElementType, ::xsd_parser_types::quick_xml::Error>
        {
            let state = ::core::mem::replace(
                &mut *self.state__,
                PackageElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::PackageElementType {
                id: self.id,
                name: self.name,
                version: self.version,
                restriction: self.restriction,
                date: self.date,
                publisher: self.publisher,
                difficulty: self.difficulty,
                logo: self.logo,
                language: self.language,
                generator: self.generator,
                contact_uri: self.contact_uri,
                tags: self.tags,
                info: self.info,
                global: self.global,
                rounds: self.rounds,
            })
        }
    }
    #[derive(Debug)]
    pub struct ParameterTypeDeserializer {
        name: ::core::option::Option<::std::string::String>,
        type_: ::core::option::Option<::std::string::String>,
        text_before: ::core::option::Option<::xsd_parser_types::xml::Text>,
        item: ::std::vec::Vec<::xsd_parser_types::xml::Mixed<super::ParameterTypeItemElementType>>,
        param: ::std::vec::Vec<::xsd_parser_types::xml::Mixed<super::ParameterType>>,
        number_set: ::core::option::Option<
            ::xsd_parser_types::xml::Mixed<super::ParameterTypeNumberSetElementType>,
        >,
        state__: ::std::boxed::Box<ParameterTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ParameterTypeDeserializerState {
        Init__ , TextBefore (:: core :: option :: Option << :: xsd_parser_types :: xml :: Text as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer >) , Item (:: core :: option :: Option << :: xsd_parser_types :: xml :: Mixed < super :: ParameterTypeItemElementType > as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer >) , Param (:: core :: option :: Option << :: xsd_parser_types :: xml :: Mixed < super :: ParameterType > as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer >) , NumberSet (:: core :: option :: Option << :: xsd_parser_types :: xml :: Mixed < super :: ParameterTypeNumberSetElementType > as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer >) , Done__ , Unknown__ , }
    impl ParameterTypeDeserializer {
        fn from_bytes_start(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            bytes_start: &::xsd_parser_types::quick_xml::BytesStart<'_>,
        ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
            let mut name: ::core::option::Option<::std::string::String> = None;
            let mut type_: ::core::option::Option<::std::string::String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_UNNAMED_2),
                    Some(b"name")
                ) {
                    helper.read_attrib(&mut name, b"name", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_UNNAMED_2),
                    Some(b"type")
                ) {
                    helper.read_attrib(&mut type_, b"type", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                name: name,
                type_: type_,
                text_before: None,
                item: ::std::vec::Vec::new(),
                param: ::std::vec::Vec::new(),
                number_set: None,
                state__: ::std::boxed::Box::new(ParameterTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            state: ParameterTypeDeserializerState,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            use ParameterTypeDeserializerState as S;
            match state {
                S::TextBefore(Some(deserializer)) => {
                    self.store_text_before(deserializer.finish(helper)?)?
                }
                S::Item(Some(deserializer)) => self.store_item(deserializer.finish(helper)?)?,
                S::Param(Some(deserializer)) => self.store_param(deserializer.finish(helper)?)?,
                S::NumberSet(Some(deserializer)) => {
                    self.store_number_set(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_text_before(
            &mut self,
            value: ::xsd_parser_types::xml::Text,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            if self.text_before.is_some() {
                Err(::xsd_parser_types::quick_xml::ErrorKind::DuplicateElement(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(b"text_before"),
                ))?;
            }
            self.text_before = Some(value);
            Ok(())
        }
        fn store_item(
            &mut self,
            value: ::xsd_parser_types::xml::Mixed<super::ParameterTypeItemElementType>,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            self.item.push(value);
            Ok(())
        }
        fn store_param(
            &mut self,
            value: ::xsd_parser_types::xml::Mixed<super::ParameterType>,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            self.param.push(value);
            Ok(())
        }
        fn store_number_set(
            &mut self,
            value: ::xsd_parser_types::xml::Mixed<super::ParameterTypeNumberSetElementType>,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            if self.number_set.is_some() {
                Err(::xsd_parser_types::quick_xml::ErrorKind::DuplicateElement(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(b"numberSet"),
                ))?;
            }
            self.number_set = Some(value);
            Ok(())
        }
        fn handle_text_before<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<
                'de,
                ::xsd_parser_types::xml::Text,
            >,
            fallback: &mut ::core::option::Option<ParameterTypeDeserializerState>,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use ParameterTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::TextBefore(None));
                *self.state__ = S::Item(None);
                return Ok(
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                        event, allow_any,
                    ),
                );
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_text_before(data)?;
                    *self.state__ = S::Item(None);
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::TextBefore(Some(deserializer)));
                    *self.state__ = S::Item(None);
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
        fn handle_item<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<
                'de,
                ::xsd_parser_types::xml::Mixed<super::ParameterTypeItemElementType>,
            >,
            fallback: &mut ::core::option::Option<ParameterTypeDeserializerState>,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use ParameterTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Item(None));
                *self.state__ = S::Param(None);
                return Ok(
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                        event, allow_any,
                    ),
                );
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_item(data)?;
                    *self.state__ = S::Item(None);
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Item(Some(deserializer)));
                    *self.state__ = S::Item(None);
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
        fn handle_param<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<
                'de,
                ::xsd_parser_types::xml::Mixed<super::ParameterType>,
            >,
            fallback: &mut ::core::option::Option<ParameterTypeDeserializerState>,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use ParameterTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Param(None));
                *self.state__ = S::NumberSet(None);
                return Ok(
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                        event, allow_any,
                    ),
                );
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_param(data)?;
                    *self.state__ = S::Param(None);
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Param(Some(deserializer)));
                    *self.state__ = S::Param(None);
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
        fn handle_number_set<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<
                'de,
                ::xsd_parser_types::xml::Mixed<super::ParameterTypeNumberSetElementType>,
            >,
            fallback: &mut ::core::option::Option<ParameterTypeDeserializerState>,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use ParameterTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::NumberSet(None));
                *self.state__ = S::Done__;
                return Ok(
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                        event, allow_any,
                    ),
                );
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_number_set(data)?;
                    *self.state__ = S::Done__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::NumberSet(Some(deserializer)));
                    *self.state__ = S::Done__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
    }
    impl<'de> ::xsd_parser_types::quick_xml::Deserializer<'de, super::ParameterType>
        for ParameterTypeDeserializer
    {
        fn init(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<'de, super::ParameterType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<'de, super::ParameterType> {
            use ParameterTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = ::core::mem::replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::TextBefore(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_text_before(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                allow_any,
                            } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (S::Item(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_item(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                allow_any,
                            } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (S::Param(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_param(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                allow_any,
                            } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (S::NumberSet(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_number_set(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                allow_any,
                            } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (_, ::xsd_parser_types::quick_xml::Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                            artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(
                                self.finish(helper)?,
                            ),
                            event: ::xsd_parser_types::quick_xml::DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::TextBefore(None);
                        event
                    }
                    (S::TextBefore(None), event) => {
                        let output = < :: xsd_parser_types :: xml :: Text as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: init (helper , event) ? ;
                        match self.handle_text_before(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                allow_any,
                            } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (
                        S::Item(None),
                        event @ (::xsd_parser_types::quick_xml::Event::Start(_)
                        | ::xsd_parser_types::quick_xml::Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_UNNAMED_2),
                            b"item",
                            false,
                        )?;
                        match self.handle_item(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                allow_any,
                            } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (
                        S::Param(None),
                        event @ (::xsd_parser_types::quick_xml::Event::Start(_)
                        | ::xsd_parser_types::quick_xml::Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_UNNAMED_2),
                            b"param",
                            false,
                        )?;
                        match self.handle_param(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                allow_any,
                            } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (
                        S::NumberSet(None),
                        event @ (::xsd_parser_types::quick_xml::Event::Start(_)
                        | ::xsd_parser_types::quick_xml::Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_UNNAMED_2),
                            b"numberSet",
                            false,
                        )?;
                        match self.handle_number_set(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                allow_any,
                            } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (
                            ::xsd_parser_types::quick_xml::DeserializerEvent::Continue(event),
                            allow_any_element,
                        );
                    }
                    (
                        state,
                        ::xsd_parser_types::quick_xml::Event::Text(_)
                        | ::xsd_parser_types::quick_xml::Event::CData(_),
                    ) => {
                        *self.state__ = state;
                        break (
                            ::xsd_parser_types::quick_xml::DeserializerEvent::None,
                            false,
                        );
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (
                            ::xsd_parser_types::quick_xml::DeserializerEvent::Break(event),
                            false,
                        );
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        ) -> ::core::result::Result<super::ParameterType, ::xsd_parser_types::quick_xml::Error>
        {
            let state = ::core::mem::replace(
                &mut *self.state__,
                ParameterTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::ParameterType {
                name: self.name,
                type_: self.type_,
                text_before: self.text_before,
                item: self.item,
                param: self.param,
                number_set: self.number_set,
            })
        }
    }
    #[derive(Debug)]
    pub struct ParametersTypeDeserializer {
        param: ::std::vec::Vec<super::ParameterType>,
        state__: ::std::boxed::Box<ParametersTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ParametersTypeDeserializerState {
        Init__ , Param (:: core :: option :: Option << super :: ParameterType as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer >) , Done__ , Unknown__ , }
    impl ParametersTypeDeserializer {
        fn from_bytes_start(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            bytes_start: &::xsd_parser_types::quick_xml::BytesStart<'_>,
        ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                param: ::std::vec::Vec::new(),
                state__: ::std::boxed::Box::new(ParametersTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            state: ParametersTypeDeserializerState,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            use ParametersTypeDeserializerState as S;
            match state {
                S::Param(Some(deserializer)) => self.store_param(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_param(
            &mut self,
            value: super::ParameterType,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            self.param.push(value);
            Ok(())
        }
        fn handle_param<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<'de, super::ParameterType>,
            fallback: &mut ::core::option::Option<ParametersTypeDeserializerState>,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use ParametersTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                if matches!(&fallback, Some(S::Init__)) {
                    return Ok(::xsd_parser_types::quick_xml::ElementHandlerOutput::break_(
                        event, allow_any,
                    ));
                } else if self.param.len() < 1usize {
                    fallback.get_or_insert(S::Param(None));
                    return Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::return_to_root(
                            event, allow_any,
                        ),
                    );
                } else {
                    fallback.get_or_insert(S::Param(None));
                    *self.state__ = S::Done__;
                    return Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    );
                }
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_param(data)?;
                    *self.state__ = S::Param(None);
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Param(Some(deserializer)));
                    *self.state__ = S::Param(None);
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
    }
    impl<'de> ::xsd_parser_types::quick_xml::Deserializer<'de, super::ParametersType>
        for ParametersTypeDeserializer
    {
        fn init(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<'de, super::ParametersType> {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<'de, super::ParametersType> {
            use ParametersTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = ::core::mem::replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Param(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_param(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                allow_any,
                            } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (_, ::xsd_parser_types::quick_xml::Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                            artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(
                                self.finish(helper)?,
                            ),
                            event: ::xsd_parser_types::quick_xml::DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::Param(None);
                        event
                    }
                    (
                        S::Param(None),
                        event @ (::xsd_parser_types::quick_xml::Event::Start(_)
                        | ::xsd_parser_types::quick_xml::Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_UNNAMED_2),
                            b"param",
                            false,
                        )?;
                        match self.handle_param(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                allow_any,
                            } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (
                            ::xsd_parser_types::quick_xml::DeserializerEvent::Continue(event),
                            allow_any_element,
                        );
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (
                            ::xsd_parser_types::quick_xml::DeserializerEvent::Break(event),
                            false,
                        );
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        ) -> ::core::result::Result<super::ParametersType, ::xsd_parser_types::quick_xml::Error>
        {
            let state = ::core::mem::replace(
                &mut *self.state__,
                ParametersTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::ParametersType {
                param: helper.finish_vec(1usize, None, self.param)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct InfoTypeAuthorsElementTypeDeserializer {
        author: ::std::vec::Vec<::std::string::String>,
        state__: ::std::boxed::Box<InfoTypeAuthorsElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum InfoTypeAuthorsElementTypeDeserializerState {
        Init__ , Author (:: core :: option :: Option << :: std :: string :: String as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer >) , Done__ , Unknown__ , }
    impl InfoTypeAuthorsElementTypeDeserializer {
        fn from_bytes_start(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            bytes_start: &::xsd_parser_types::quick_xml::BytesStart<'_>,
        ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                author: ::std::vec::Vec::new(),
                state__: ::std::boxed::Box::new(
                    InfoTypeAuthorsElementTypeDeserializerState::Init__,
                ),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            state: InfoTypeAuthorsElementTypeDeserializerState,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            use InfoTypeAuthorsElementTypeDeserializerState as S;
            match state {
                S::Author(Some(deserializer)) => self.store_author(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_author(
            &mut self,
            value: ::std::string::String,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            self.author.push(value);
            Ok(())
        }
        fn handle_author<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<'de, ::std::string::String>,
            fallback: &mut ::core::option::Option<InfoTypeAuthorsElementTypeDeserializerState>,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use InfoTypeAuthorsElementTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Author(None));
                *self.state__ = S::Done__;
                return Ok(
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                        event, allow_any,
                    ),
                );
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_author(data)?;
                    *self.state__ = S::Author(None);
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Author(Some(deserializer)));
                    *self.state__ = S::Author(None);
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
    }
    impl<'de> ::xsd_parser_types::quick_xml::Deserializer<'de, super::InfoTypeAuthorsElementType>
        for InfoTypeAuthorsElementTypeDeserializer
    {
        fn init(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<'de, super::InfoTypeAuthorsElementType>
        {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<'de, super::InfoTypeAuthorsElementType>
        {
            use InfoTypeAuthorsElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = ::core::mem::replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Author(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_author(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                allow_any,
                            } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (_, ::xsd_parser_types::quick_xml::Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                            artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(
                                self.finish(helper)?,
                            ),
                            event: ::xsd_parser_types::quick_xml::DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::Author(None);
                        event
                    }
                    (
                        S::Author(None),
                        event @ (::xsd_parser_types::quick_xml::Event::Start(_)
                        | ::xsd_parser_types::quick_xml::Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_UNNAMED_2),
                            b"author",
                            false,
                        )?;
                        match self.handle_author(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                allow_any,
                            } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (
                            ::xsd_parser_types::quick_xml::DeserializerEvent::Continue(event),
                            allow_any_element,
                        );
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (
                            ::xsd_parser_types::quick_xml::DeserializerEvent::Break(event),
                            false,
                        );
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        ) -> ::core::result::Result<
            super::InfoTypeAuthorsElementType,
            ::xsd_parser_types::quick_xml::Error,
        > {
            let state = ::core::mem::replace(
                &mut *self.state__,
                InfoTypeAuthorsElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::InfoTypeAuthorsElementType {
                author: self.author,
            })
        }
    }
    #[derive(Debug)]
    pub struct InfoTypeSourcesElementTypeDeserializer {
        source: ::std::vec::Vec<::std::string::String>,
        state__: ::std::boxed::Box<InfoTypeSourcesElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum InfoTypeSourcesElementTypeDeserializerState {
        Init__ , Source (:: core :: option :: Option << :: std :: string :: String as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer >) , Done__ , Unknown__ , }
    impl InfoTypeSourcesElementTypeDeserializer {
        fn from_bytes_start(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            bytes_start: &::xsd_parser_types::quick_xml::BytesStart<'_>,
        ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                source: ::std::vec::Vec::new(),
                state__: ::std::boxed::Box::new(
                    InfoTypeSourcesElementTypeDeserializerState::Init__,
                ),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            state: InfoTypeSourcesElementTypeDeserializerState,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            use InfoTypeSourcesElementTypeDeserializerState as S;
            match state {
                S::Source(Some(deserializer)) => self.store_source(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_source(
            &mut self,
            value: ::std::string::String,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            self.source.push(value);
            Ok(())
        }
        fn handle_source<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<'de, ::std::string::String>,
            fallback: &mut ::core::option::Option<InfoTypeSourcesElementTypeDeserializerState>,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use InfoTypeSourcesElementTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Source(None));
                *self.state__ = S::Done__;
                return Ok(
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                        event, allow_any,
                    ),
                );
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_source(data)?;
                    *self.state__ = S::Source(None);
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Source(Some(deserializer)));
                    *self.state__ = S::Source(None);
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
    }
    impl<'de> ::xsd_parser_types::quick_xml::Deserializer<'de, super::InfoTypeSourcesElementType>
        for InfoTypeSourcesElementTypeDeserializer
    {
        fn init(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<'de, super::InfoTypeSourcesElementType>
        {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<'de, super::InfoTypeSourcesElementType>
        {
            use InfoTypeSourcesElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = ::core::mem::replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Source(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_source(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                allow_any,
                            } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (_, ::xsd_parser_types::quick_xml::Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                            artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(
                                self.finish(helper)?,
                            ),
                            event: ::xsd_parser_types::quick_xml::DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::Source(None);
                        event
                    }
                    (
                        S::Source(None),
                        event @ (::xsd_parser_types::quick_xml::Event::Start(_)
                        | ::xsd_parser_types::quick_xml::Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_UNNAMED_2),
                            b"source",
                            false,
                        )?;
                        match self.handle_source(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                allow_any,
                            } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (
                            ::xsd_parser_types::quick_xml::DeserializerEvent::Continue(event),
                            allow_any_element,
                        );
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (
                            ::xsd_parser_types::quick_xml::DeserializerEvent::Break(event),
                            false,
                        );
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        ) -> ::core::result::Result<
            super::InfoTypeSourcesElementType,
            ::xsd_parser_types::quick_xml::Error,
        > {
            let state = ::core::mem::replace(
                &mut *self.state__,
                InfoTypeSourcesElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::InfoTypeSourcesElementType {
                source: self.source,
            })
        }
    }
    #[derive(Debug)]
    pub struct PackageTagsElementTypeDeserializer {
        tag: ::std::vec::Vec<::std::string::String>,
        state__: ::std::boxed::Box<PackageTagsElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum PackageTagsElementTypeDeserializerState {
        Init__ , Tag (:: core :: option :: Option << :: std :: string :: String as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer >) , Done__ , Unknown__ , }
    impl PackageTagsElementTypeDeserializer {
        fn from_bytes_start(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            bytes_start: &::xsd_parser_types::quick_xml::BytesStart<'_>,
        ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                tag: ::std::vec::Vec::new(),
                state__: ::std::boxed::Box::new(PackageTagsElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            state: PackageTagsElementTypeDeserializerState,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            use PackageTagsElementTypeDeserializerState as S;
            match state {
                S::Tag(Some(deserializer)) => self.store_tag(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_tag(
            &mut self,
            value: ::std::string::String,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            self.tag.push(value);
            Ok(())
        }
        fn handle_tag<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<'de, ::std::string::String>,
            fallback: &mut ::core::option::Option<PackageTagsElementTypeDeserializerState>,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use PackageTagsElementTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Tag(None));
                *self.state__ = S::Done__;
                return Ok(
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                        event, allow_any,
                    ),
                );
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_tag(data)?;
                    *self.state__ = S::Tag(None);
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Tag(Some(deserializer)));
                    *self.state__ = S::Tag(None);
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
    }
    impl<'de> ::xsd_parser_types::quick_xml::Deserializer<'de, super::PackageTagsElementType>
        for PackageTagsElementTypeDeserializer
    {
        fn init(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<'de, super::PackageTagsElementType>
        {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<'de, super::PackageTagsElementType>
        {
            use PackageTagsElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = ::core::mem::replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Tag(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_tag(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                allow_any,
                            } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (_, ::xsd_parser_types::quick_xml::Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                            artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(
                                self.finish(helper)?,
                            ),
                            event: ::xsd_parser_types::quick_xml::DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::Tag(None);
                        event
                    }
                    (
                        S::Tag(None),
                        event @ (::xsd_parser_types::quick_xml::Event::Start(_)
                        | ::xsd_parser_types::quick_xml::Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_UNNAMED_2),
                            b"tag",
                            false,
                        )?;
                        match self.handle_tag(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                allow_any,
                            } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (
                            ::xsd_parser_types::quick_xml::DeserializerEvent::Continue(event),
                            allow_any_element,
                        );
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (
                            ::xsd_parser_types::quick_xml::DeserializerEvent::Break(event),
                            false,
                        );
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        ) -> ::core::result::Result<
            super::PackageTagsElementType,
            ::xsd_parser_types::quick_xml::Error,
        > {
            let state = ::core::mem::replace(
                &mut *self.state__,
                PackageTagsElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::PackageTagsElementType { tag: self.tag })
        }
    }
    #[derive(Debug)]
    pub struct PackageGlobalElementTypeDeserializer {
        authors: ::core::option::Option<super::PackageGlobalAuthorsElementType>,
        sources: ::core::option::Option<super::PackageGlobalSourcesElementType>,
        state__: ::std::boxed::Box<PackageGlobalElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum PackageGlobalElementTypeDeserializerState {
        Init__ , Next__ , Authors (< super :: PackageGlobalAuthorsElementType as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer) , Sources (< super :: PackageGlobalSourcesElementType as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer) , Unknown__ , }
    impl PackageGlobalElementTypeDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
            fallback: &mut ::core::option::Option<PackageGlobalElementTypeDeserializerState>,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            if let ::xsd_parser_types::quick_xml::Event::Start(x)
            | ::xsd_parser_types::quick_xml::Event::Empty(x) = &event
            {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_UNNAMED_2),
                    Some(b"Authors")
                ) {
                    let output = < super :: PackageGlobalAuthorsElementType as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: init (helper , event) ? ;
                    return self.handle_authors(helper, output, &mut *fallback);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_UNNAMED_2),
                    Some(b"Sources")
                ) {
                    let output = < super :: PackageGlobalSourcesElementType as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: init (helper , event) ? ;
                    return self.handle_sources(helper, output, &mut *fallback);
                }
            }
            *self.state__ = fallback
                .take()
                .unwrap_or(PackageGlobalElementTypeDeserializerState::Init__);
            Ok(::xsd_parser_types::quick_xml::ElementHandlerOutput::return_to_parent(event, false))
        }
        fn from_bytes_start(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            bytes_start: &::xsd_parser_types::quick_xml::BytesStart<'_>,
        ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                authors: None,
                sources: None,
                state__: ::std::boxed::Box::new(PackageGlobalElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            state: PackageGlobalElementTypeDeserializerState,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            use PackageGlobalElementTypeDeserializerState as S;
            match state {
                S::Authors(deserializer) => self.store_authors(deserializer.finish(helper)?)?,
                S::Sources(deserializer) => self.store_sources(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_authors(
            &mut self,
            value: super::PackageGlobalAuthorsElementType,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            if self.authors.is_some() {
                Err(::xsd_parser_types::quick_xml::ErrorKind::DuplicateElement(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(b"Authors"),
                ))?;
            }
            self.authors = Some(value);
            Ok(())
        }
        fn store_sources(
            &mut self,
            value: super::PackageGlobalSourcesElementType,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            if self.sources.is_some() {
                Err(::xsd_parser_types::quick_xml::ErrorKind::DuplicateElement(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(b"Sources"),
                ))?;
            }
            self.sources = Some(value);
            Ok(())
        }
        fn handle_authors<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<
                'de,
                super::PackageGlobalAuthorsElementType,
            >,
            fallback: &mut ::core::option::Option<PackageGlobalElementTypeDeserializerState>,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use PackageGlobalElementTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Next__;
                return Ok(
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput::return_to_root(
                        event, allow_any,
                    ),
                );
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_authors(data)?;
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Authors(deserializer));
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
        fn handle_sources<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<
                'de,
                super::PackageGlobalSourcesElementType,
            >,
            fallback: &mut ::core::option::Option<PackageGlobalElementTypeDeserializerState>,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use PackageGlobalElementTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Next__;
                return Ok(
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput::return_to_root(
                        event, allow_any,
                    ),
                );
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_sources(data)?;
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Sources(deserializer));
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
    }
    impl<'de> ::xsd_parser_types::quick_xml::Deserializer<'de, super::PackageGlobalElementType>
        for PackageGlobalElementTypeDeserializer
    {
        fn init(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<'de, super::PackageGlobalElementType>
        {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<'de, super::PackageGlobalElementType>
        {
            use PackageGlobalElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = ::core::mem::replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Authors(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_authors(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                ..
                            } => event,
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (S::Sources(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_sources(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                ..
                            } => event,
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (_, ::xsd_parser_types::quick_xml::Event::End(_)) => {
                        return Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                            artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(
                                self.finish(helper)?,
                            ),
                            event: ::xsd_parser_types::quick_xml::DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        match self.find_suitable(helper, event, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                ..
                            } => event,
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        ) -> ::core::result::Result<
            super::PackageGlobalElementType,
            ::xsd_parser_types::quick_xml::Error,
        > {
            let state = ::core::mem::replace(
                &mut *self.state__,
                PackageGlobalElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::PackageGlobalElementType {
                authors: helper.finish_element("Authors", self.authors)?,
                sources: helper.finish_element("Sources", self.sources)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct PackageRoundsElementTypeDeserializer {
        round: ::std::vec::Vec<super::PackageRoundsRoundElementType>,
        state__: ::std::boxed::Box<PackageRoundsElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum PackageRoundsElementTypeDeserializerState {
        Init__ , Round (:: core :: option :: Option << super :: PackageRoundsRoundElementType as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer >) , Done__ , Unknown__ , }
    impl PackageRoundsElementTypeDeserializer {
        fn from_bytes_start(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            bytes_start: &::xsd_parser_types::quick_xml::BytesStart<'_>,
        ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                round: ::std::vec::Vec::new(),
                state__: ::std::boxed::Box::new(PackageRoundsElementTypeDeserializerState::Init__),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            state: PackageRoundsElementTypeDeserializerState,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            use PackageRoundsElementTypeDeserializerState as S;
            match state {
                S::Round(Some(deserializer)) => self.store_round(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_round(
            &mut self,
            value: super::PackageRoundsRoundElementType,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            self.round.push(value);
            Ok(())
        }
        fn handle_round<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<
                'de,
                super::PackageRoundsRoundElementType,
            >,
            fallback: &mut ::core::option::Option<PackageRoundsElementTypeDeserializerState>,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use PackageRoundsElementTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Round(None));
                *self.state__ = S::Done__;
                return Ok(
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                        event, allow_any,
                    ),
                );
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_round(data)?;
                    *self.state__ = S::Round(None);
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Round(Some(deserializer)));
                    *self.state__ = S::Round(None);
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
    }
    impl<'de> ::xsd_parser_types::quick_xml::Deserializer<'de, super::PackageRoundsElementType>
        for PackageRoundsElementTypeDeserializer
    {
        fn init(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<'de, super::PackageRoundsElementType>
        {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<'de, super::PackageRoundsElementType>
        {
            use PackageRoundsElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = ::core::mem::replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Round(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_round(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                allow_any,
                            } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (_, ::xsd_parser_types::quick_xml::Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                            artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(
                                self.finish(helper)?,
                            ),
                            event: ::xsd_parser_types::quick_xml::DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::Round(None);
                        event
                    }
                    (
                        S::Round(None),
                        event @ (::xsd_parser_types::quick_xml::Event::Start(_)
                        | ::xsd_parser_types::quick_xml::Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_UNNAMED_2),
                            b"round",
                            false,
                        )?;
                        match self.handle_round(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                allow_any,
                            } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (
                            ::xsd_parser_types::quick_xml::DeserializerEvent::Continue(event),
                            allow_any_element,
                        );
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (
                            ::xsd_parser_types::quick_xml::DeserializerEvent::Break(event),
                            false,
                        );
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        ) -> ::core::result::Result<
            super::PackageRoundsElementType,
            ::xsd_parser_types::quick_xml::Error,
        > {
            let state = ::core::mem::replace(
                &mut *self.state__,
                PackageRoundsElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::PackageRoundsElementType { round: self.round })
        }
    }
    #[derive(Debug)]
    pub struct ParameterTypeItemElementTypeDeserializer {
        type_: ::core::option::Option<::std::string::String>,
        is_ref: ::core::option::Option<::std::string::String>,
        placement: ::core::option::Option<::std::string::String>,
        duration: ::core::option::Option<::std::string::String>,
        wait_for_finish: ::core::option::Option<::std::string::String>,
        content: ::core::option::Option<::std::string::String>,
        state__: ::std::boxed::Box<ParameterTypeItemElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ParameterTypeItemElementTypeDeserializerState {
        Init__ , Content__ (< :: std :: string :: String as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer) , Unknown__ , }
    impl ParameterTypeItemElementTypeDeserializer {
        fn from_bytes_start(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            bytes_start: &::xsd_parser_types::quick_xml::BytesStart<'_>,
        ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
            let mut type_: ::core::option::Option<::std::string::String> = None;
            let mut is_ref: ::core::option::Option<::std::string::String> = None;
            let mut placement: ::core::option::Option<::std::string::String> = None;
            let mut duration: ::core::option::Option<::std::string::String> = None;
            let mut wait_for_finish: ::core::option::Option<::std::string::String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_UNNAMED_2),
                    Some(b"type")
                ) {
                    helper.read_attrib(&mut type_, b"type", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_UNNAMED_2),
                    Some(b"isRef")
                ) {
                    helper.read_attrib(&mut is_ref, b"isRef", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_UNNAMED_2),
                    Some(b"placement")
                ) {
                    helper.read_attrib(&mut placement, b"placement", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_UNNAMED_2),
                    Some(b"duration")
                ) {
                    helper.read_attrib(&mut duration, b"duration", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_UNNAMED_2),
                    Some(b"waitForFinish")
                ) {
                    helper.read_attrib(&mut wait_for_finish, b"waitForFinish", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                type_: type_,
                is_ref: is_ref,
                placement: placement,
                duration: duration,
                wait_for_finish: wait_for_finish,
                content: None,
                state__: ::std::boxed::Box::new(
                    ParameterTypeItemElementTypeDeserializerState::Init__,
                ),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            state: ParameterTypeItemElementTypeDeserializerState,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            if let ParameterTypeItemElementTypeDeserializerState::Content__(deserializer) = state {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(
            &mut self,
            value: ::std::string::String,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            if self.content.is_some() {
                Err(::xsd_parser_types::quick_xml::ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<'de, ::std::string::String>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<
            'de,
            super::ParameterTypeItemElementType,
        > {
            use ParameterTypeItemElementTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => {
                    Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                        artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::None,
                        event,
                        allow_any,
                    })
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(helper)?;
                    Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                        artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Content__(deserializer);
                    Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                        artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(
                            self,
                        ),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de> ::xsd_parser_types::quick_xml::Deserializer<'de, super::ParameterTypeItemElementType>
        for ParameterTypeItemElementTypeDeserializer
    {
        fn init(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<
            'de,
            super::ParameterTypeItemElementType,
        > {
            let (::xsd_parser_types::quick_xml::Event::Start(x)
            | ::xsd_parser_types::quick_xml::Event::Empty(x)) = &event
            else {
                return Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                    artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::None,
                    event: ::xsd_parser_types::quick_xml::DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(helper, x)?.next(helper, event)
        }
        fn next(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<
            'de,
            super::ParameterTypeItemElementType,
        > {
            use ParameterTypeItemElementTypeDeserializerState as S;
            match ::core::mem::replace(&mut *self.state__, S::Unknown__) {
                S::Unknown__ => unreachable!(),
                S::Init__ => {
                    let output =
                        ::xsd_parser_types::quick_xml::ContentDeserializer::init(helper, event)?;
                    self.handle_content(helper, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(helper, event)?;
                    self.handle_content(helper, output)
                }
            }
        }
        fn finish(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        ) -> ::core::result::Result<
            super::ParameterTypeItemElementType,
            ::xsd_parser_types::quick_xml::Error,
        > {
            let state = ::core::mem::replace(
                &mut *self.state__,
                ParameterTypeItemElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::ParameterTypeItemElementType {
                type_: self.type_,
                is_ref: self.is_ref,
                placement: self.placement,
                duration: self.duration,
                wait_for_finish: self.wait_for_finish,
                content: helper.finish_content(self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct ParameterTypeNumberSetElementTypeDeserializer {
        minimum: ::core::option::Option<::core::primitive::i32>,
        maximum: ::core::option::Option<::core::primitive::i32>,
        step: ::core::option::Option<::core::primitive::i32>,
        content: ::core::option::Option<::std::string::String>,
        state__: ::std::boxed::Box<ParameterTypeNumberSetElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum ParameterTypeNumberSetElementTypeDeserializerState {
        Init__ , Content__ (< :: std :: string :: String as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer) , Unknown__ , }
    impl ParameterTypeNumberSetElementTypeDeserializer {
        fn from_bytes_start(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            bytes_start: &::xsd_parser_types::quick_xml::BytesStart<'_>,
        ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
            let mut minimum: ::core::option::Option<::core::primitive::i32> = None;
            let mut maximum: ::core::option::Option<::core::primitive::i32> = None;
            let mut step: ::core::option::Option<::core::primitive::i32> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_UNNAMED_2),
                    Some(b"minimum")
                ) {
                    helper.read_attrib(&mut minimum, b"minimum", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_UNNAMED_2),
                    Some(b"maximum")
                ) {
                    helper.read_attrib(&mut maximum, b"maximum", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_UNNAMED_2),
                    Some(b"step")
                ) {
                    helper.read_attrib(&mut step, b"step", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                minimum: minimum,
                maximum: maximum,
                step: step,
                content: None,
                state__: ::std::boxed::Box::new(
                    ParameterTypeNumberSetElementTypeDeserializerState::Init__,
                ),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            state: ParameterTypeNumberSetElementTypeDeserializerState,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            if let ParameterTypeNumberSetElementTypeDeserializerState::Content__(deserializer) =
                state
            {
                self.store_content(deserializer.finish(helper)?)?;
            }
            Ok(())
        }
        fn store_content(
            &mut self,
            value: ::std::string::String,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            if self.content.is_some() {
                Err(::xsd_parser_types::quick_xml::ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<'de, ::std::string::String>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<
            'de,
            super::ParameterTypeNumberSetElementType,
        > {
            use ParameterTypeNumberSetElementTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => {
                    Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                        artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::None,
                        event,
                        allow_any,
                    })
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(helper)?;
                    Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                        artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Content__(deserializer);
                    Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                        artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(
                            self,
                        ),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de>
        ::xsd_parser_types::quick_xml::Deserializer<'de, super::ParameterTypeNumberSetElementType>
        for ParameterTypeNumberSetElementTypeDeserializer
    {
        fn init(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<
            'de,
            super::ParameterTypeNumberSetElementType,
        > {
            let (::xsd_parser_types::quick_xml::Event::Start(x)
            | ::xsd_parser_types::quick_xml::Event::Empty(x)) = &event
            else {
                return Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                    artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::None,
                    event: ::xsd_parser_types::quick_xml::DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(helper, x)?.next(helper, event)
        }
        fn next(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<
            'de,
            super::ParameterTypeNumberSetElementType,
        > {
            use ParameterTypeNumberSetElementTypeDeserializerState as S;
            match ::core::mem::replace(&mut *self.state__, S::Unknown__) {
                S::Unknown__ => unreachable!(),
                S::Init__ => {
                    let output =
                        ::xsd_parser_types::quick_xml::ContentDeserializer::init(helper, event)?;
                    self.handle_content(helper, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(helper, event)?;
                    self.handle_content(helper, output)
                }
            }
        }
        fn finish(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        ) -> ::core::result::Result<
            super::ParameterTypeNumberSetElementType,
            ::xsd_parser_types::quick_xml::Error,
        > {
            let state = ::core::mem::replace(
                &mut *self.state__,
                ParameterTypeNumberSetElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::ParameterTypeNumberSetElementType {
                minimum: self.minimum,
                maximum: self.maximum,
                step: self.step,
                content: helper.finish_content(self.content)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct PackageGlobalAuthorsElementTypeDeserializer {
        id: ::std::string::String,
        name: ::core::option::Option<::std::string::String>,
        second_name: ::core::option::Option<::std::string::String>,
        surname: ::core::option::Option<::std::string::String>,
        country: ::core::option::Option<::std::string::String>,
        city: ::core::option::Option<::std::string::String>,
        state__: ::std::boxed::Box<PackageGlobalAuthorsElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum PackageGlobalAuthorsElementTypeDeserializerState {
        Init__ , Next__ , Name (< :: std :: string :: String as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer) , SecondName (< :: std :: string :: String as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer) , Surname (< :: std :: string :: String as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer) , Country (< :: std :: string :: String as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer) , City (< :: std :: string :: String as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer) , Unknown__ , }
    impl PackageGlobalAuthorsElementTypeDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
            fallback: &mut ::core::option::Option<PackageGlobalAuthorsElementTypeDeserializerState>,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            if let ::xsd_parser_types::quick_xml::Event::Start(x)
            | ::xsd_parser_types::quick_xml::Event::Empty(x) = &event
            {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_UNNAMED_2),
                    Some(b"Name")
                ) {
                    let output = < :: std :: string :: String as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: init (helper , event) ? ;
                    return self.handle_name(helper, output, &mut *fallback);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_UNNAMED_2),
                    Some(b"SecondName")
                ) {
                    let output = < :: std :: string :: String as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: init (helper , event) ? ;
                    return self.handle_second_name(helper, output, &mut *fallback);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_UNNAMED_2),
                    Some(b"Surname")
                ) {
                    let output = < :: std :: string :: String as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: init (helper , event) ? ;
                    return self.handle_surname(helper, output, &mut *fallback);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_UNNAMED_2),
                    Some(b"Country")
                ) {
                    let output = < :: std :: string :: String as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: init (helper , event) ? ;
                    return self.handle_country(helper, output, &mut *fallback);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_UNNAMED_2),
                    Some(b"City")
                ) {
                    let output = < :: std :: string :: String as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: init (helper , event) ? ;
                    return self.handle_city(helper, output, &mut *fallback);
                }
            }
            *self.state__ = fallback
                .take()
                .unwrap_or(PackageGlobalAuthorsElementTypeDeserializerState::Init__);
            Ok(::xsd_parser_types::quick_xml::ElementHandlerOutput::return_to_parent(event, false))
        }
        fn from_bytes_start(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            bytes_start: &::xsd_parser_types::quick_xml::BytesStart<'_>,
        ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
            let mut id: ::core::option::Option<::std::string::String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_UNNAMED_2),
                    Some(b"id")
                ) {
                    helper.read_attrib(&mut id, b"id", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                id: id.ok_or_else(|| {
                    ::xsd_parser_types::quick_xml::ErrorKind::MissingAttribute("id".into())
                })?,
                name: None,
                second_name: None,
                surname: None,
                country: None,
                city: None,
                state__: ::std::boxed::Box::new(
                    PackageGlobalAuthorsElementTypeDeserializerState::Init__,
                ),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            state: PackageGlobalAuthorsElementTypeDeserializerState,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            use PackageGlobalAuthorsElementTypeDeserializerState as S;
            match state {
                S::Name(deserializer) => self.store_name(deserializer.finish(helper)?)?,
                S::SecondName(deserializer) => {
                    self.store_second_name(deserializer.finish(helper)?)?
                }
                S::Surname(deserializer) => self.store_surname(deserializer.finish(helper)?)?,
                S::Country(deserializer) => self.store_country(deserializer.finish(helper)?)?,
                S::City(deserializer) => self.store_city(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_name(
            &mut self,
            value: ::std::string::String,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            if self.name.is_some() {
                Err(::xsd_parser_types::quick_xml::ErrorKind::DuplicateElement(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(b"Name"),
                ))?;
            }
            self.name = Some(value);
            Ok(())
        }
        fn store_second_name(
            &mut self,
            value: ::std::string::String,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            if self.second_name.is_some() {
                Err(::xsd_parser_types::quick_xml::ErrorKind::DuplicateElement(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(b"SecondName"),
                ))?;
            }
            self.second_name = Some(value);
            Ok(())
        }
        fn store_surname(
            &mut self,
            value: ::std::string::String,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            if self.surname.is_some() {
                Err(::xsd_parser_types::quick_xml::ErrorKind::DuplicateElement(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(b"Surname"),
                ))?;
            }
            self.surname = Some(value);
            Ok(())
        }
        fn store_country(
            &mut self,
            value: ::std::string::String,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            if self.country.is_some() {
                Err(::xsd_parser_types::quick_xml::ErrorKind::DuplicateElement(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(b"Country"),
                ))?;
            }
            self.country = Some(value);
            Ok(())
        }
        fn store_city(
            &mut self,
            value: ::std::string::String,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            if self.city.is_some() {
                Err(::xsd_parser_types::quick_xml::ErrorKind::DuplicateElement(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(b"City"),
                ))?;
            }
            self.city = Some(value);
            Ok(())
        }
        fn handle_name<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<'de, ::std::string::String>,
            fallback: &mut ::core::option::Option<PackageGlobalAuthorsElementTypeDeserializerState>,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use PackageGlobalAuthorsElementTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Next__;
                return Ok(
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput::return_to_root(
                        event, allow_any,
                    ),
                );
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_name(data)?;
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Name(deserializer));
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
        fn handle_second_name<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<'de, ::std::string::String>,
            fallback: &mut ::core::option::Option<PackageGlobalAuthorsElementTypeDeserializerState>,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use PackageGlobalAuthorsElementTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Next__;
                return Ok(
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput::return_to_root(
                        event, allow_any,
                    ),
                );
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_second_name(data)?;
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::SecondName(deserializer));
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
        fn handle_surname<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<'de, ::std::string::String>,
            fallback: &mut ::core::option::Option<PackageGlobalAuthorsElementTypeDeserializerState>,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use PackageGlobalAuthorsElementTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Next__;
                return Ok(
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput::return_to_root(
                        event, allow_any,
                    ),
                );
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_surname(data)?;
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Surname(deserializer));
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
        fn handle_country<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<'de, ::std::string::String>,
            fallback: &mut ::core::option::Option<PackageGlobalAuthorsElementTypeDeserializerState>,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use PackageGlobalAuthorsElementTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Next__;
                return Ok(
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput::return_to_root(
                        event, allow_any,
                    ),
                );
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_country(data)?;
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Country(deserializer));
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
        fn handle_city<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<'de, ::std::string::String>,
            fallback: &mut ::core::option::Option<PackageGlobalAuthorsElementTypeDeserializerState>,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use PackageGlobalAuthorsElementTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Next__;
                return Ok(
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput::return_to_root(
                        event, allow_any,
                    ),
                );
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_city(data)?;
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::City(deserializer));
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
    }
    impl<'de>
        ::xsd_parser_types::quick_xml::Deserializer<'de, super::PackageGlobalAuthorsElementType>
        for PackageGlobalAuthorsElementTypeDeserializer
    {
        fn init(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<
            'de,
            super::PackageGlobalAuthorsElementType,
        > {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<
            'de,
            super::PackageGlobalAuthorsElementType,
        > {
            use PackageGlobalAuthorsElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = ::core::mem::replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Name(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_name(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                ..
                            } => event,
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (S::SecondName(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_second_name(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                ..
                            } => event,
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (S::Surname(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_surname(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                ..
                            } => event,
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (S::Country(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_country(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                ..
                            } => event,
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (S::City(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_city(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                ..
                            } => event,
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (_, ::xsd_parser_types::quick_xml::Event::End(_)) => {
                        return Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                            artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(
                                self.finish(helper)?,
                            ),
                            event: ::xsd_parser_types::quick_xml::DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        match self.find_suitable(helper, event, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                ..
                            } => event,
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        ) -> ::core::result::Result<
            super::PackageGlobalAuthorsElementType,
            ::xsd_parser_types::quick_xml::Error,
        > {
            let state = ::core::mem::replace(
                &mut *self.state__,
                PackageGlobalAuthorsElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::PackageGlobalAuthorsElementType {
                id: self.id,
                name: helper.finish_element("Name", self.name)?,
                second_name: helper.finish_element("SecondName", self.second_name)?,
                surname: helper.finish_element("Surname", self.surname)?,
                country: helper.finish_element("Country", self.country)?,
                city: helper.finish_element("City", self.city)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct PackageGlobalSourcesElementTypeDeserializer {
        id: ::std::string::String,
        author: ::core::option::Option<::std::string::String>,
        title: ::core::option::Option<::std::string::String>,
        year: ::core::option::Option<::std::string::String>,
        publish: ::core::option::Option<::std::string::String>,
        city: ::core::option::Option<::std::string::String>,
        state__: ::std::boxed::Box<PackageGlobalSourcesElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum PackageGlobalSourcesElementTypeDeserializerState {
        Init__ , Next__ , Author (< :: std :: string :: String as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer) , Title (< :: std :: string :: String as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer) , Year (< :: std :: string :: String as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer) , Publish (< :: std :: string :: String as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer) , City (< :: std :: string :: String as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer) , Unknown__ , }
    impl PackageGlobalSourcesElementTypeDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
            fallback: &mut ::core::option::Option<PackageGlobalSourcesElementTypeDeserializerState>,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            if let ::xsd_parser_types::quick_xml::Event::Start(x)
            | ::xsd_parser_types::quick_xml::Event::Empty(x) = &event
            {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_UNNAMED_2),
                    Some(b"Author")
                ) {
                    let output = < :: std :: string :: String as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: init (helper , event) ? ;
                    return self.handle_author(helper, output, &mut *fallback);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_UNNAMED_2),
                    Some(b"Title")
                ) {
                    let output = < :: std :: string :: String as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: init (helper , event) ? ;
                    return self.handle_title(helper, output, &mut *fallback);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_UNNAMED_2),
                    Some(b"Year")
                ) {
                    let output = < :: std :: string :: String as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: init (helper , event) ? ;
                    return self.handle_year(helper, output, &mut *fallback);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_UNNAMED_2),
                    Some(b"Publish")
                ) {
                    let output = < :: std :: string :: String as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: init (helper , event) ? ;
                    return self.handle_publish(helper, output, &mut *fallback);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_UNNAMED_2),
                    Some(b"City")
                ) {
                    let output = < :: std :: string :: String as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: init (helper , event) ? ;
                    return self.handle_city(helper, output, &mut *fallback);
                }
            }
            *self.state__ = fallback
                .take()
                .unwrap_or(PackageGlobalSourcesElementTypeDeserializerState::Init__);
            Ok(::xsd_parser_types::quick_xml::ElementHandlerOutput::return_to_parent(event, false))
        }
        fn from_bytes_start(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            bytes_start: &::xsd_parser_types::quick_xml::BytesStart<'_>,
        ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
            let mut id: ::core::option::Option<::std::string::String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_UNNAMED_2),
                    Some(b"id")
                ) {
                    helper.read_attrib(&mut id, b"id", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                id: id.ok_or_else(|| {
                    ::xsd_parser_types::quick_xml::ErrorKind::MissingAttribute("id".into())
                })?,
                author: None,
                title: None,
                year: None,
                publish: None,
                city: None,
                state__: ::std::boxed::Box::new(
                    PackageGlobalSourcesElementTypeDeserializerState::Init__,
                ),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            state: PackageGlobalSourcesElementTypeDeserializerState,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            use PackageGlobalSourcesElementTypeDeserializerState as S;
            match state {
                S::Author(deserializer) => self.store_author(deserializer.finish(helper)?)?,
                S::Title(deserializer) => self.store_title(deserializer.finish(helper)?)?,
                S::Year(deserializer) => self.store_year(deserializer.finish(helper)?)?,
                S::Publish(deserializer) => self.store_publish(deserializer.finish(helper)?)?,
                S::City(deserializer) => self.store_city(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_author(
            &mut self,
            value: ::std::string::String,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            if self.author.is_some() {
                Err(::xsd_parser_types::quick_xml::ErrorKind::DuplicateElement(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(b"Author"),
                ))?;
            }
            self.author = Some(value);
            Ok(())
        }
        fn store_title(
            &mut self,
            value: ::std::string::String,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            if self.title.is_some() {
                Err(::xsd_parser_types::quick_xml::ErrorKind::DuplicateElement(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(b"Title"),
                ))?;
            }
            self.title = Some(value);
            Ok(())
        }
        fn store_year(
            &mut self,
            value: ::std::string::String,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            if self.year.is_some() {
                Err(::xsd_parser_types::quick_xml::ErrorKind::DuplicateElement(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(b"Year"),
                ))?;
            }
            self.year = Some(value);
            Ok(())
        }
        fn store_publish(
            &mut self,
            value: ::std::string::String,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            if self.publish.is_some() {
                Err(::xsd_parser_types::quick_xml::ErrorKind::DuplicateElement(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(b"Publish"),
                ))?;
            }
            self.publish = Some(value);
            Ok(())
        }
        fn store_city(
            &mut self,
            value: ::std::string::String,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            if self.city.is_some() {
                Err(::xsd_parser_types::quick_xml::ErrorKind::DuplicateElement(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(b"City"),
                ))?;
            }
            self.city = Some(value);
            Ok(())
        }
        fn handle_author<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<'de, ::std::string::String>,
            fallback: &mut ::core::option::Option<PackageGlobalSourcesElementTypeDeserializerState>,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use PackageGlobalSourcesElementTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Next__;
                return Ok(
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput::return_to_root(
                        event, allow_any,
                    ),
                );
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_author(data)?;
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Author(deserializer));
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
        fn handle_title<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<'de, ::std::string::String>,
            fallback: &mut ::core::option::Option<PackageGlobalSourcesElementTypeDeserializerState>,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use PackageGlobalSourcesElementTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Next__;
                return Ok(
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput::return_to_root(
                        event, allow_any,
                    ),
                );
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_title(data)?;
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Title(deserializer));
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
        fn handle_year<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<'de, ::std::string::String>,
            fallback: &mut ::core::option::Option<PackageGlobalSourcesElementTypeDeserializerState>,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use PackageGlobalSourcesElementTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Next__;
                return Ok(
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput::return_to_root(
                        event, allow_any,
                    ),
                );
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_year(data)?;
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Year(deserializer));
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
        fn handle_publish<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<'de, ::std::string::String>,
            fallback: &mut ::core::option::Option<PackageGlobalSourcesElementTypeDeserializerState>,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use PackageGlobalSourcesElementTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Next__;
                return Ok(
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput::return_to_root(
                        event, allow_any,
                    ),
                );
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_publish(data)?;
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Publish(deserializer));
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
        fn handle_city<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<'de, ::std::string::String>,
            fallback: &mut ::core::option::Option<PackageGlobalSourcesElementTypeDeserializerState>,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use PackageGlobalSourcesElementTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Next__;
                return Ok(
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput::return_to_root(
                        event, allow_any,
                    ),
                );
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_city(data)?;
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::City(deserializer));
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
    }
    impl<'de>
        ::xsd_parser_types::quick_xml::Deserializer<'de, super::PackageGlobalSourcesElementType>
        for PackageGlobalSourcesElementTypeDeserializer
    {
        fn init(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<
            'de,
            super::PackageGlobalSourcesElementType,
        > {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<
            'de,
            super::PackageGlobalSourcesElementType,
        > {
            use PackageGlobalSourcesElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = ::core::mem::replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Author(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_author(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                ..
                            } => event,
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (S::Title(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_title(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                ..
                            } => event,
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (S::Year(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_year(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                ..
                            } => event,
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (S::Publish(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_publish(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                ..
                            } => event,
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (S::City(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_city(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                ..
                            } => event,
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (_, ::xsd_parser_types::quick_xml::Event::End(_)) => {
                        return Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                            artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(
                                self.finish(helper)?,
                            ),
                            event: ::xsd_parser_types::quick_xml::DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        match self.find_suitable(helper, event, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                ..
                            } => event,
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        ) -> ::core::result::Result<
            super::PackageGlobalSourcesElementType,
            ::xsd_parser_types::quick_xml::Error,
        > {
            let state = ::core::mem::replace(
                &mut *self.state__,
                PackageGlobalSourcesElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::PackageGlobalSourcesElementType {
                id: self.id,
                author: helper.finish_element("Author", self.author)?,
                title: helper.finish_element("Title", self.title)?,
                year: helper.finish_element("Year", self.year)?,
                publish: helper.finish_element("Publish", self.publish)?,
                city: helper.finish_element("City", self.city)?,
            })
        }
    }
    #[derive(Debug)]
    pub struct PackageRoundsRoundElementTypeDeserializer {
        name: ::std::string::String,
        type_: ::core::option::Option<::std::string::String>,
        info: ::core::option::Option<super::InfoType>,
        themes: ::core::option::Option<super::PackageRoundsRoundThemesElementType>,
        state__: ::std::boxed::Box<PackageRoundsRoundElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum PackageRoundsRoundElementTypeDeserializerState {
        Init__ , Next__ , Info (< super :: InfoType as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer) , Themes (< super :: PackageRoundsRoundThemesElementType as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer) , Unknown__ , }
    impl PackageRoundsRoundElementTypeDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
            fallback: &mut ::core::option::Option<PackageRoundsRoundElementTypeDeserializerState>,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            if let ::xsd_parser_types::quick_xml::Event::Start(x)
            | ::xsd_parser_types::quick_xml::Event::Empty(x) = &event
            {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_UNNAMED_2),
                    Some(b"info")
                ) {
                    let output =
                        <super::InfoType as ::xsd_parser_types::quick_xml::WithDeserializer>::init(
                            helper, event,
                        )?;
                    return self.handle_info(helper, output, &mut *fallback);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_UNNAMED_2),
                    Some(b"themes")
                ) {
                    let output = < super :: PackageRoundsRoundThemesElementType as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: init (helper , event) ? ;
                    return self.handle_themes(helper, output, &mut *fallback);
                }
            }
            *self.state__ = fallback
                .take()
                .unwrap_or(PackageRoundsRoundElementTypeDeserializerState::Init__);
            Ok(::xsd_parser_types::quick_xml::ElementHandlerOutput::return_to_parent(event, false))
        }
        fn from_bytes_start(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            bytes_start: &::xsd_parser_types::quick_xml::BytesStart<'_>,
        ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
            let mut name: ::core::option::Option<::std::string::String> = None;
            let mut type_: ::core::option::Option<::std::string::String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_UNNAMED_2),
                    Some(b"name")
                ) {
                    helper.read_attrib(&mut name, b"name", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_UNNAMED_2),
                    Some(b"type")
                ) {
                    helper.read_attrib(&mut type_, b"type", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                name: name.ok_or_else(|| {
                    ::xsd_parser_types::quick_xml::ErrorKind::MissingAttribute("name".into())
                })?,
                type_: type_,
                info: None,
                themes: None,
                state__: ::std::boxed::Box::new(
                    PackageRoundsRoundElementTypeDeserializerState::Init__,
                ),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            state: PackageRoundsRoundElementTypeDeserializerState,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            use PackageRoundsRoundElementTypeDeserializerState as S;
            match state {
                S::Info(deserializer) => self.store_info(deserializer.finish(helper)?)?,
                S::Themes(deserializer) => self.store_themes(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_info(
            &mut self,
            value: super::InfoType,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            if self.info.is_some() {
                Err(::xsd_parser_types::quick_xml::ErrorKind::DuplicateElement(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(b"info"),
                ))?;
            }
            self.info = Some(value);
            Ok(())
        }
        fn store_themes(
            &mut self,
            value: super::PackageRoundsRoundThemesElementType,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            if self.themes.is_some() {
                Err(::xsd_parser_types::quick_xml::ErrorKind::DuplicateElement(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(b"themes"),
                ))?;
            }
            self.themes = Some(value);
            Ok(())
        }
        fn handle_info<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<'de, super::InfoType>,
            fallback: &mut ::core::option::Option<PackageRoundsRoundElementTypeDeserializerState>,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use PackageRoundsRoundElementTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Next__;
                return Ok(
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput::return_to_root(
                        event, allow_any,
                    ),
                );
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_info(data)?;
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Info(deserializer));
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
        fn handle_themes<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<
                'de,
                super::PackageRoundsRoundThemesElementType,
            >,
            fallback: &mut ::core::option::Option<PackageRoundsRoundElementTypeDeserializerState>,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use PackageRoundsRoundElementTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Next__;
                return Ok(
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput::return_to_root(
                        event, allow_any,
                    ),
                );
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_themes(data)?;
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Themes(deserializer));
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
    }
    impl<'de> ::xsd_parser_types::quick_xml::Deserializer<'de, super::PackageRoundsRoundElementType>
        for PackageRoundsRoundElementTypeDeserializer
    {
        fn init(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<
            'de,
            super::PackageRoundsRoundElementType,
        > {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<
            'de,
            super::PackageRoundsRoundElementType,
        > {
            use PackageRoundsRoundElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = ::core::mem::replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Info(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_info(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                ..
                            } => event,
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (S::Themes(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_themes(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                ..
                            } => event,
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (_, ::xsd_parser_types::quick_xml::Event::End(_)) => {
                        return Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                            artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(
                                self.finish(helper)?,
                            ),
                            event: ::xsd_parser_types::quick_xml::DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        match self.find_suitable(helper, event, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                ..
                            } => event,
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        ) -> ::core::result::Result<
            super::PackageRoundsRoundElementType,
            ::xsd_parser_types::quick_xml::Error,
        > {
            let state = ::core::mem::replace(
                &mut *self.state__,
                PackageRoundsRoundElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::PackageRoundsRoundElementType {
                name: self.name,
                type_: self.type_,
                info: self.info,
                themes: self.themes,
            })
        }
    }
    #[derive(Debug)]
    pub struct PackageRoundsRoundThemesElementTypeDeserializer {
        theme: ::std::vec::Vec<super::PackageRoundsRoundThemesThemeElementType>,
        state__: ::std::boxed::Box<PackageRoundsRoundThemesElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum PackageRoundsRoundThemesElementTypeDeserializerState {
        Init__ , Theme (:: core :: option :: Option << super :: PackageRoundsRoundThemesThemeElementType as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer >) , Done__ , Unknown__ , }
    impl PackageRoundsRoundThemesElementTypeDeserializer {
        fn from_bytes_start(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            bytes_start: &::xsd_parser_types::quick_xml::BytesStart<'_>,
        ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                theme: ::std::vec::Vec::new(),
                state__: ::std::boxed::Box::new(
                    PackageRoundsRoundThemesElementTypeDeserializerState::Init__,
                ),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            state: PackageRoundsRoundThemesElementTypeDeserializerState,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            use PackageRoundsRoundThemesElementTypeDeserializerState as S;
            match state {
                S::Theme(Some(deserializer)) => self.store_theme(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_theme(
            &mut self,
            value: super::PackageRoundsRoundThemesThemeElementType,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            self.theme.push(value);
            Ok(())
        }
        fn handle_theme<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<
                'de,
                super::PackageRoundsRoundThemesThemeElementType,
            >,
            fallback: &mut ::core::option::Option<
                PackageRoundsRoundThemesElementTypeDeserializerState,
            >,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use PackageRoundsRoundThemesElementTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Theme(None));
                *self.state__ = S::Done__;
                return Ok(
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                        event, allow_any,
                    ),
                );
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_theme(data)?;
                    *self.state__ = S::Theme(None);
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Theme(Some(deserializer)));
                    *self.state__ = S::Theme(None);
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
    }
    impl<'de>
        ::xsd_parser_types::quick_xml::Deserializer<'de, super::PackageRoundsRoundThemesElementType>
        for PackageRoundsRoundThemesElementTypeDeserializer
    {
        fn init(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<
            'de,
            super::PackageRoundsRoundThemesElementType,
        > {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<
            'de,
            super::PackageRoundsRoundThemesElementType,
        > {
            use PackageRoundsRoundThemesElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = ::core::mem::replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Theme(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_theme(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                allow_any,
                            } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (_, ::xsd_parser_types::quick_xml::Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                            artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(
                                self.finish(helper)?,
                            ),
                            event: ::xsd_parser_types::quick_xml::DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::Theme(None);
                        event
                    }
                    (
                        S::Theme(None),
                        event @ (::xsd_parser_types::quick_xml::Event::Start(_)
                        | ::xsd_parser_types::quick_xml::Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_UNNAMED_2),
                            b"theme",
                            false,
                        )?;
                        match self.handle_theme(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                allow_any,
                            } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (
                            ::xsd_parser_types::quick_xml::DeserializerEvent::Continue(event),
                            allow_any_element,
                        );
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (
                            ::xsd_parser_types::quick_xml::DeserializerEvent::Break(event),
                            false,
                        );
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        ) -> ::core::result::Result<
            super::PackageRoundsRoundThemesElementType,
            ::xsd_parser_types::quick_xml::Error,
        > {
            let state = ::core::mem::replace(
                &mut *self.state__,
                PackageRoundsRoundThemesElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::PackageRoundsRoundThemesElementType { theme: self.theme })
        }
    }
    #[derive(Debug)]
    pub struct PackageRoundsRoundThemesThemeElementTypeDeserializer {
        name: ::std::string::String,
        info: ::core::option::Option<super::InfoType>,
        questions: ::core::option::Option<super::PackageRoundsRoundThemesThemeQuestionsElementType>,
        state__: ::std::boxed::Box<PackageRoundsRoundThemesThemeElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum PackageRoundsRoundThemesThemeElementTypeDeserializerState {
        Init__ , Next__ , Info (< super :: InfoType as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer) , Questions (< super :: PackageRoundsRoundThemesThemeQuestionsElementType as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer) , Unknown__ , }
    impl PackageRoundsRoundThemesThemeElementTypeDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
            fallback: &mut ::core::option::Option<
                PackageRoundsRoundThemesThemeElementTypeDeserializerState,
            >,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            if let ::xsd_parser_types::quick_xml::Event::Start(x)
            | ::xsd_parser_types::quick_xml::Event::Empty(x) = &event
            {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_UNNAMED_2),
                    Some(b"info")
                ) {
                    let output =
                        <super::InfoType as ::xsd_parser_types::quick_xml::WithDeserializer>::init(
                            helper, event,
                        )?;
                    return self.handle_info(helper, output, &mut *fallback);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_UNNAMED_2),
                    Some(b"questions")
                ) {
                    let output = < super :: PackageRoundsRoundThemesThemeQuestionsElementType as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: init (helper , event) ? ;
                    return self.handle_questions(helper, output, &mut *fallback);
                }
            }
            *self.state__ = fallback
                .take()
                .unwrap_or(PackageRoundsRoundThemesThemeElementTypeDeserializerState::Init__);
            Ok(::xsd_parser_types::quick_xml::ElementHandlerOutput::return_to_parent(event, false))
        }
        fn from_bytes_start(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            bytes_start: &::xsd_parser_types::quick_xml::BytesStart<'_>,
        ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
            let mut name: ::core::option::Option<::std::string::String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_UNNAMED_2),
                    Some(b"name")
                ) {
                    helper.read_attrib(&mut name, b"name", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok(Self {
                name: name.ok_or_else(|| {
                    ::xsd_parser_types::quick_xml::ErrorKind::MissingAttribute("name".into())
                })?,
                info: None,
                questions: None,
                state__: ::std::boxed::Box::new(
                    PackageRoundsRoundThemesThemeElementTypeDeserializerState::Init__,
                ),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            state: PackageRoundsRoundThemesThemeElementTypeDeserializerState,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            use PackageRoundsRoundThemesThemeElementTypeDeserializerState as S;
            match state {
                S::Info(deserializer) => self.store_info(deserializer.finish(helper)?)?,
                S::Questions(deserializer) => self.store_questions(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_info(
            &mut self,
            value: super::InfoType,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            if self.info.is_some() {
                Err(::xsd_parser_types::quick_xml::ErrorKind::DuplicateElement(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(b"info"),
                ))?;
            }
            self.info = Some(value);
            Ok(())
        }
        fn store_questions(
            &mut self,
            value: super::PackageRoundsRoundThemesThemeQuestionsElementType,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            if self.questions.is_some() {
                Err(::xsd_parser_types::quick_xml::ErrorKind::DuplicateElement(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(b"questions"),
                ))?;
            }
            self.questions = Some(value);
            Ok(())
        }
        fn handle_info<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<'de, super::InfoType>,
            fallback: &mut ::core::option::Option<
                PackageRoundsRoundThemesThemeElementTypeDeserializerState,
            >,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use PackageRoundsRoundThemesThemeElementTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Next__;
                return Ok(
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput::return_to_root(
                        event, allow_any,
                    ),
                );
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_info(data)?;
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Info(deserializer));
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
        fn handle_questions<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<
                'de,
                super::PackageRoundsRoundThemesThemeQuestionsElementType,
            >,
            fallback: &mut ::core::option::Option<
                PackageRoundsRoundThemesThemeElementTypeDeserializerState,
            >,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use PackageRoundsRoundThemesThemeElementTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Next__;
                return Ok(
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput::return_to_root(
                        event, allow_any,
                    ),
                );
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_questions(data)?;
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Questions(deserializer));
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
    }
    impl<'de>
        ::xsd_parser_types::quick_xml::Deserializer<
            'de,
            super::PackageRoundsRoundThemesThemeElementType,
        > for PackageRoundsRoundThemesThemeElementTypeDeserializer
    {
        fn init(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<
            'de,
            super::PackageRoundsRoundThemesThemeElementType,
        > {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<
            'de,
            super::PackageRoundsRoundThemesThemeElementType,
        > {
            use PackageRoundsRoundThemesThemeElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = ::core::mem::replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Info(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_info(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                ..
                            } => event,
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (S::Questions(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_questions(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                ..
                            } => event,
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (_, ::xsd_parser_types::quick_xml::Event::End(_)) => {
                        return Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                            artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(
                                self.finish(helper)?,
                            ),
                            event: ::xsd_parser_types::quick_xml::DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        match self.find_suitable(helper, event, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                ..
                            } => event,
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        ) -> ::core::result::Result<
            super::PackageRoundsRoundThemesThemeElementType,
            ::xsd_parser_types::quick_xml::Error,
        > {
            let state = ::core::mem::replace(
                &mut *self.state__,
                PackageRoundsRoundThemesThemeElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::PackageRoundsRoundThemesThemeElementType {
                name: self.name,
                info: self.info,
                questions: self.questions,
            })
        }
    }
    #[derive(Debug)]
    pub struct PackageRoundsRoundThemesThemeQuestionsElementTypeDeserializer {
        question: ::std::vec::Vec<super::PackageRoundsRoundThemesThemeQuestionsQuestionElementType>,
        state__:
            ::std::boxed::Box<PackageRoundsRoundThemesThemeQuestionsElementTypeDeserializerState>,
    }
    #[derive(Debug)]
    enum PackageRoundsRoundThemesThemeQuestionsElementTypeDeserializerState {
        Init__ , Question (:: core :: option :: Option << super :: PackageRoundsRoundThemesThemeQuestionsQuestionElementType as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer >) , Done__ , Unknown__ , }
    impl PackageRoundsRoundThemesThemeQuestionsElementTypeDeserializer {
        fn from_bytes_start(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            bytes_start: &::xsd_parser_types::quick_xml::BytesStart<'_>,
        ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok(Self {
                question: ::std::vec::Vec::new(),
                state__: ::std::boxed::Box::new(
                    PackageRoundsRoundThemesThemeQuestionsElementTypeDeserializerState::Init__,
                ),
            })
        }
        fn finish_state(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            state: PackageRoundsRoundThemesThemeQuestionsElementTypeDeserializerState,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            use PackageRoundsRoundThemesThemeQuestionsElementTypeDeserializerState as S;
            match state {
                S::Question(Some(deserializer)) => {
                    self.store_question(deserializer.finish(helper)?)?
                }
                _ => (),
            }
            Ok(())
        }
        fn store_question(
            &mut self,
            value: super::PackageRoundsRoundThemesThemeQuestionsQuestionElementType,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            self.question.push(value);
            Ok(())
        }
        fn handle_question<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<
                'de,
                super::PackageRoundsRoundThemesThemeQuestionsQuestionElementType,
            >,
            fallback: &mut ::core::option::Option<
                PackageRoundsRoundThemesThemeQuestionsElementTypeDeserializerState,
            >,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use PackageRoundsRoundThemesThemeQuestionsElementTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Question(None));
                *self.state__ = S::Done__;
                return Ok(
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                        event, allow_any,
                    ),
                );
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_question(data)?;
                    *self.state__ = S::Question(None);
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Question(Some(deserializer)));
                    *self.state__ = S::Question(None);
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
    }
    impl<'de>
        ::xsd_parser_types::quick_xml::Deserializer<
            'de,
            super::PackageRoundsRoundThemesThemeQuestionsElementType,
        > for PackageRoundsRoundThemesThemeQuestionsElementTypeDeserializer
    {
        fn init(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<
            'de,
            super::PackageRoundsRoundThemesThemeQuestionsElementType,
        > {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<
            'de,
            super::PackageRoundsRoundThemesThemeQuestionsElementType,
        > {
            use PackageRoundsRoundThemesThemeQuestionsElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = ::core::mem::replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Question(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_question(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                allow_any,
                            } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (_, ::xsd_parser_types::quick_xml::Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                            artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(
                                self.finish(helper)?,
                            ),
                            event: ::xsd_parser_types::quick_xml::DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::Question(None);
                        event
                    }
                    (
                        S::Question(None),
                        event @ (::xsd_parser_types::quick_xml::Event::Start(_)
                        | ::xsd_parser_types::quick_xml::Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_UNNAMED_2),
                            b"question",
                            false,
                        )?;
                        match self.handle_question(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                allow_any,
                            } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (
                            ::xsd_parser_types::quick_xml::DeserializerEvent::Continue(event),
                            allow_any_element,
                        );
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (
                            ::xsd_parser_types::quick_xml::DeserializerEvent::Break(event),
                            false,
                        );
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        ) -> ::core::result::Result<
            super::PackageRoundsRoundThemesThemeQuestionsElementType,
            ::xsd_parser_types::quick_xml::Error,
        > {
            let state = ::core::mem::replace(
                &mut *self.state__,
                PackageRoundsRoundThemesThemeQuestionsElementTypeDeserializerState::Unknown__,
            );
            self.finish_state(helper, state)?;
            Ok(super::PackageRoundsRoundThemesThemeQuestionsElementType {
                question: self.question,
            })
        }
    }
    #[derive(Debug)]
    pub struct PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeDeserializer {
        price: ::core::primitive::i32,
        type_: ::core::option::Option<::std::string::String>,
        info: ::core::option::Option<super::InfoType>,
        type_name: ::core::option::Option<
            super::PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameElementType,
        >,
        scenario: ::core::option::Option<
            super::PackageRoundsRoundThemesThemeQuestionsQuestionScenarioElementType,
        >,
        script: ::core::option::Option<
            super::PackageRoundsRoundThemesThemeQuestionsQuestionScriptElementType,
        >,
        params: ::core::option::Option<super::ParametersType>,
        right: ::core::option::Option<
            super::PackageRoundsRoundThemesThemeQuestionsQuestionRightElementType,
        >,
        wrong: ::core::option::Option<
            super::PackageRoundsRoundThemesThemeQuestionsQuestionRightElementType,
        >,
        state__: ::std::boxed::Box<
            PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeDeserializerState,
        >,
    }
    #[derive(Debug)]
    enum PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeDeserializerState {
        Init__ , Next__ , Info (< super :: InfoType as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer) , TypeName (< super :: PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameElementType as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer) , Scenario (< super :: PackageRoundsRoundThemesThemeQuestionsQuestionScenarioElementType as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer) , Script (< super :: PackageRoundsRoundThemesThemeQuestionsQuestionScriptElementType as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer) , Params (< super :: ParametersType as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer) , Right (< super :: PackageRoundsRoundThemesThemeQuestionsQuestionRightElementType as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer) , Wrong (< super :: PackageRoundsRoundThemesThemeQuestionsQuestionRightElementType as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer) , Unknown__ , }
    impl PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeDeserializer {
        fn find_suitable<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
            fallback: &mut ::core::option::Option<
                PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeDeserializerState,
            >,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            if let ::xsd_parser_types::quick_xml::Event::Start(x)
            | ::xsd_parser_types::quick_xml::Event::Empty(x) = &event
            {
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_UNNAMED_2),
                    Some(b"info")
                ) {
                    let output =
                        <super::InfoType as ::xsd_parser_types::quick_xml::WithDeserializer>::init(
                            helper, event,
                        )?;
                    return self.handle_info(helper, output, &mut *fallback);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_UNNAMED_2),
                    Some(b"type_name")
                ) {
                    let output = < super :: PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameElementType as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: init (helper , event) ? ;
                    return self.handle_type_name(helper, output, &mut *fallback);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_UNNAMED_2),
                    Some(b"scenario")
                ) {
                    let output = < super :: PackageRoundsRoundThemesThemeQuestionsQuestionScenarioElementType as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: init (helper , event) ? ;
                    return self.handle_scenario(helper, output, &mut *fallback);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_UNNAMED_2),
                    Some(b"script")
                ) {
                    let output = < super :: PackageRoundsRoundThemesThemeQuestionsQuestionScriptElementType as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: init (helper , event) ? ;
                    return self.handle_script(helper, output, &mut *fallback);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_UNNAMED_2),
                    Some(b"params")
                ) {
                    let output = < super :: ParametersType as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: init (helper , event) ? ;
                    return self.handle_params(helper, output, &mut *fallback);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_UNNAMED_2),
                    Some(b"right")
                ) {
                    let output = < super :: PackageRoundsRoundThemesThemeQuestionsQuestionRightElementType as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: init (helper , event) ? ;
                    return self.handle_right(helper, output, &mut *fallback);
                }
                if matches!(
                    helper.resolve_local_name(x.name(), &super::NS_UNNAMED_2),
                    Some(b"wrong")
                ) {
                    let output = < super :: PackageRoundsRoundThemesThemeQuestionsQuestionRightElementType as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: init (helper , event) ? ;
                    return self.handle_wrong(helper, output, &mut *fallback);
                }
            }
            *self.state__ = fallback.take().unwrap_or(
                PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeDeserializerState::Init__,
            );
            Ok(::xsd_parser_types::quick_xml::ElementHandlerOutput::return_to_parent(event, false))
        }
        fn from_bytes_start(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            bytes_start: &::xsd_parser_types::quick_xml::BytesStart<'_>,
        ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
            let mut price: ::core::option::Option<::core::primitive::i32> = None;
            let mut type_: ::core::option::Option<::std::string::String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_UNNAMED_2),
                    Some(b"price")
                ) {
                    helper.read_attrib(&mut price, b"price", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_UNNAMED_2),
                    Some(b"type")
                ) {
                    helper.read_attrib(&mut type_, b"type", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok (Self { price : price . ok_or_else (|| :: xsd_parser_types :: quick_xml :: ErrorKind :: MissingAttribute ("price" . into ())) ? , type_ : type_ , info : None , type_name : None , scenario : None , script : None , params : None , right : None , wrong : None , state__ : :: std :: boxed :: Box :: new (PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeDeserializerState :: Init__) , })
        }
        fn finish_state(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            state: PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeDeserializerState,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            use PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeDeserializerState as S;
            match state {
                S::Info(deserializer) => self.store_info(deserializer.finish(helper)?)?,
                S::TypeName(deserializer) => self.store_type_name(deserializer.finish(helper)?)?,
                S::Scenario(deserializer) => self.store_scenario(deserializer.finish(helper)?)?,
                S::Script(deserializer) => self.store_script(deserializer.finish(helper)?)?,
                S::Params(deserializer) => self.store_params(deserializer.finish(helper)?)?,
                S::Right(deserializer) => self.store_right(deserializer.finish(helper)?)?,
                S::Wrong(deserializer) => self.store_wrong(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_info(
            &mut self,
            value: super::InfoType,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            if self.info.is_some() {
                Err(::xsd_parser_types::quick_xml::ErrorKind::DuplicateElement(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(b"info"),
                ))?;
            }
            self.info = Some(value);
            Ok(())
        }
        fn store_type_name(
            &mut self,
            value: super::PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameElementType,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            if self.type_name.is_some() {
                Err(::xsd_parser_types::quick_xml::ErrorKind::DuplicateElement(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(b"type_name"),
                ))?;
            }
            self.type_name = Some(value);
            Ok(())
        }
        fn store_scenario(
            &mut self,
            value: super::PackageRoundsRoundThemesThemeQuestionsQuestionScenarioElementType,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            if self.scenario.is_some() {
                Err(::xsd_parser_types::quick_xml::ErrorKind::DuplicateElement(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(b"scenario"),
                ))?;
            }
            self.scenario = Some(value);
            Ok(())
        }
        fn store_script(
            &mut self,
            value: super::PackageRoundsRoundThemesThemeQuestionsQuestionScriptElementType,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            if self.script.is_some() {
                Err(::xsd_parser_types::quick_xml::ErrorKind::DuplicateElement(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(b"script"),
                ))?;
            }
            self.script = Some(value);
            Ok(())
        }
        fn store_params(
            &mut self,
            value: super::ParametersType,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            if self.params.is_some() {
                Err(::xsd_parser_types::quick_xml::ErrorKind::DuplicateElement(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(b"params"),
                ))?;
            }
            self.params = Some(value);
            Ok(())
        }
        fn store_right(
            &mut self,
            value: super::PackageRoundsRoundThemesThemeQuestionsQuestionRightElementType,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            if self.right.is_some() {
                Err(::xsd_parser_types::quick_xml::ErrorKind::DuplicateElement(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(b"right"),
                ))?;
            }
            self.right = Some(value);
            Ok(())
        }
        fn store_wrong(
            &mut self,
            value: super::PackageRoundsRoundThemesThemeQuestionsQuestionRightElementType,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            if self.wrong.is_some() {
                Err(::xsd_parser_types::quick_xml::ErrorKind::DuplicateElement(
                    ::xsd_parser_types::quick_xml::RawByteStr::from_slice(b"wrong"),
                ))?;
            }
            self.wrong = Some(value);
            Ok(())
        }
        fn handle_info<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<'de, super::InfoType>,
            fallback: &mut ::core::option::Option<
                PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeDeserializerState,
            >,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Next__;
                return Ok(
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput::return_to_root(
                        event, allow_any,
                    ),
                );
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_info(data)?;
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Info(deserializer));
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
        fn handle_type_name<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<
                'de,
                super::PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameElementType,
            >,
            fallback: &mut ::core::option::Option<
                PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeDeserializerState,
            >,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Next__;
                return Ok(
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput::return_to_root(
                        event, allow_any,
                    ),
                );
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_type_name(data)?;
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::TypeName(deserializer));
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
        fn handle_scenario<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<
                'de,
                super::PackageRoundsRoundThemesThemeQuestionsQuestionScenarioElementType,
            >,
            fallback: &mut ::core::option::Option<
                PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeDeserializerState,
            >,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Next__;
                return Ok(
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput::return_to_root(
                        event, allow_any,
                    ),
                );
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_scenario(data)?;
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Scenario(deserializer));
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
        fn handle_script<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<
                'de,
                super::PackageRoundsRoundThemesThemeQuestionsQuestionScriptElementType,
            >,
            fallback: &mut ::core::option::Option<
                PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeDeserializerState,
            >,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Next__;
                return Ok(
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput::return_to_root(
                        event, allow_any,
                    ),
                );
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_script(data)?;
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Script(deserializer));
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
        fn handle_params<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<'de, super::ParametersType>,
            fallback: &mut ::core::option::Option<
                PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeDeserializerState,
            >,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Next__;
                return Ok(
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput::return_to_root(
                        event, allow_any,
                    ),
                );
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_params(data)?;
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Params(deserializer));
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
        fn handle_right<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<
                'de,
                super::PackageRoundsRoundThemesThemeQuestionsQuestionRightElementType,
            >,
            fallback: &mut ::core::option::Option<
                PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeDeserializerState,
            >,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Next__;
                return Ok(
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput::return_to_root(
                        event, allow_any,
                    ),
                );
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_right(data)?;
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Right(deserializer));
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
        fn handle_wrong<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<
                'de,
                super::PackageRoundsRoundThemesThemeQuestionsQuestionRightElementType,
            >,
            fallback: &mut ::core::option::Option<
                PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeDeserializerState,
            >,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                *self.state__ = S::Next__;
                return Ok(
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput::return_to_root(
                        event, allow_any,
                    ),
                );
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_wrong(data)?;
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Wrong(deserializer));
                    *self.state__ = S::Next__;
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
    }
    impl<'de>
        ::xsd_parser_types::quick_xml::Deserializer<
            'de,
            super::PackageRoundsRoundThemesThemeQuestionsQuestionElementType,
        > for PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeDeserializer
    {
        fn init(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<
            'de,
            super::PackageRoundsRoundThemesThemeQuestionsQuestionElementType,
        > {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<
            'de,
            super::PackageRoundsRoundThemesThemeQuestionsQuestionElementType,
        > {
            use PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let (event, allow_any) = loop {
                let state = ::core::mem::replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Info(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_info(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                ..
                            } => event,
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (S::TypeName(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_type_name(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                ..
                            } => event,
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (S::Scenario(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_scenario(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                ..
                            } => event,
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (S::Script(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_script(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                ..
                            } => event,
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (S::Params(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_params(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                ..
                            } => event,
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (S::Right(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_right(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                ..
                            } => event,
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (S::Wrong(deserializer), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_wrong(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                ..
                            } => event,
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (_, ::xsd_parser_types::quick_xml::Event::End(_)) => {
                        return Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                            artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(
                                self.finish(helper)?,
                            ),
                            event: ::xsd_parser_types::quick_xml::DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (state @ (S::Init__ | S::Next__), event) => {
                        fallback.get_or_insert(state);
                        match self.find_suitable(helper, event, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                ..
                            } => event,
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        ) -> ::core::result::Result<
            super::PackageRoundsRoundThemesThemeQuestionsQuestionElementType,
            ::xsd_parser_types::quick_xml::Error,
        > {
            let state = :: core :: mem :: replace (& mut * self . state__ , PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeDeserializerState :: Unknown__) ;
            self.finish_state(helper, state)?;
            Ok(
                super::PackageRoundsRoundThemesThemeQuestionsQuestionElementType {
                    price: self.price,
                    type_: self.type_,
                    info: self.info,
                    type_name: self.type_name,
                    scenario: self.scenario,
                    script: self.script,
                    params: self.params,
                    right: helper.finish_element("right", self.right)?,
                    wrong: self.wrong,
                },
            )
        }
    }
    #[derive(Debug)]
    pub struct PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameElementTypeDeserializer {
        name: ::std::string::String,
        param: ::std::vec::Vec<
            super::PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameParamElementType,
        >,
        state__: ::std::boxed::Box<
            PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameElementTypeDeserializerState,
        >,
    }
    #[derive(Debug)]
    enum PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameElementTypeDeserializerState {
        Init__ , Param (:: core :: option :: Option << super :: PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameParamElementType as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer >) , Done__ , Unknown__ , }
    impl PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameElementTypeDeserializer {
        fn from_bytes_start(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            bytes_start: &::xsd_parser_types::quick_xml::BytesStart<'_>,
        ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
            let mut name: ::core::option::Option<::std::string::String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_UNNAMED_2),
                    Some(b"name")
                ) {
                    helper.read_attrib(&mut name, b"name", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok (Self { name : name . ok_or_else (|| :: xsd_parser_types :: quick_xml :: ErrorKind :: MissingAttribute ("name" . into ())) ? , param : :: std :: vec :: Vec :: new () , state__ : :: std :: boxed :: Box :: new (PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameElementTypeDeserializerState :: Init__) , })
        }
        fn finish_state(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            state : PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameElementTypeDeserializerState,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            use PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameElementTypeDeserializerState as S;
            match state {
                S::Param(Some(deserializer)) => self.store_param(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_param(
            &mut self,
            value: super::PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameParamElementType,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            self.param.push(value);
            Ok(())
        }
        fn handle_param<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<
                'de,
                super::PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameParamElementType,
            >,
            fallback: &mut ::core::option::Option<
                PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameElementTypeDeserializerState,
            >,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameElementTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Param(None));
                *self.state__ = S::Done__;
                return Ok(
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                        event, allow_any,
                    ),
                );
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_param(data)?;
                    *self.state__ = S::Param(None);
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Param(Some(deserializer)));
                    *self.state__ = S::Param(None);
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
    }
    impl<'de>
        ::xsd_parser_types::quick_xml::Deserializer<
            'de,
            super::PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameElementType,
        > for PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameElementTypeDeserializer
    {
        fn init(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<
            'de,
            super::PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameElementType,
        > {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<
            'de,
            super::PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameElementType,
        > {
            use PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = ::core::mem::replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Param(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_param(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                allow_any,
                            } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (_, ::xsd_parser_types::quick_xml::Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                            artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(
                                self.finish(helper)?,
                            ),
                            event: ::xsd_parser_types::quick_xml::DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::Param(None);
                        event
                    }
                    (
                        S::Param(None),
                        event @ (::xsd_parser_types::quick_xml::Event::Start(_)
                        | ::xsd_parser_types::quick_xml::Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_UNNAMED_2),
                            b"param",
                            false,
                        )?;
                        match self.handle_param(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                allow_any,
                            } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (
                            ::xsd_parser_types::quick_xml::DeserializerEvent::Continue(event),
                            allow_any_element,
                        );
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (
                            ::xsd_parser_types::quick_xml::DeserializerEvent::Break(event),
                            false,
                        );
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        ) -> ::core::result::Result<
            super::PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameElementType,
            ::xsd_parser_types::quick_xml::Error,
        > {
            let state = :: core :: mem :: replace (& mut * self . state__ , PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameElementTypeDeserializerState :: Unknown__) ;
            self.finish_state(helper, state)?;
            Ok(
                super::PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameElementType {
                    name: self.name,
                    param: self.param,
                },
            )
        }
    }
    #[derive(Debug)]
    pub struct PackageRoundsRoundThemesThemeQuestionsQuestionScenarioElementTypeDeserializer {
        atom: ::std::vec::Vec<
            super::PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomElementType,
        >,
        state__: ::std::boxed::Box<
            PackageRoundsRoundThemesThemeQuestionsQuestionScenarioElementTypeDeserializerState,
        >,
    }
    #[derive(Debug)]
    enum PackageRoundsRoundThemesThemeQuestionsQuestionScenarioElementTypeDeserializerState {
        Init__ , Atom (:: core :: option :: Option << super :: PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomElementType as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer >) , Done__ , Unknown__ , }
    impl PackageRoundsRoundThemesThemeQuestionsQuestionScenarioElementTypeDeserializer {
        fn from_bytes_start(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            bytes_start: &::xsd_parser_types::quick_xml::BytesStart<'_>,
        ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok (Self { atom : :: std :: vec :: Vec :: new () , state__ : :: std :: boxed :: Box :: new (PackageRoundsRoundThemesThemeQuestionsQuestionScenarioElementTypeDeserializerState :: Init__) , })
        }
        fn finish_state(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            state : PackageRoundsRoundThemesThemeQuestionsQuestionScenarioElementTypeDeserializerState,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            use PackageRoundsRoundThemesThemeQuestionsQuestionScenarioElementTypeDeserializerState as S;
            match state {
                S::Atom(Some(deserializer)) => self.store_atom(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_atom(
            &mut self,
            value: super::PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomElementType,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            self.atom.push(value);
            Ok(())
        }
        fn handle_atom<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<
                'de,
                super::PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomElementType,
            >,
            fallback: &mut ::core::option::Option<
                PackageRoundsRoundThemesThemeQuestionsQuestionScenarioElementTypeDeserializerState,
            >,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use PackageRoundsRoundThemesThemeQuestionsQuestionScenarioElementTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Atom(None));
                *self.state__ = S::Done__;
                return Ok(
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                        event, allow_any,
                    ),
                );
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_atom(data)?;
                    *self.state__ = S::Atom(None);
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Atom(Some(deserializer)));
                    *self.state__ = S::Atom(None);
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
    }
    impl<'de>
        ::xsd_parser_types::quick_xml::Deserializer<
            'de,
            super::PackageRoundsRoundThemesThemeQuestionsQuestionScenarioElementType,
        > for PackageRoundsRoundThemesThemeQuestionsQuestionScenarioElementTypeDeserializer
    {
        fn init(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<
            'de,
            super::PackageRoundsRoundThemesThemeQuestionsQuestionScenarioElementType,
        > {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<
            'de,
            super::PackageRoundsRoundThemesThemeQuestionsQuestionScenarioElementType,
        > {
            use PackageRoundsRoundThemesThemeQuestionsQuestionScenarioElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = ::core::mem::replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Atom(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_atom(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                allow_any,
                            } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (_, ::xsd_parser_types::quick_xml::Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                            artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(
                                self.finish(helper)?,
                            ),
                            event: ::xsd_parser_types::quick_xml::DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::Atom(None);
                        event
                    }
                    (
                        S::Atom(None),
                        event @ (::xsd_parser_types::quick_xml::Event::Start(_)
                        | ::xsd_parser_types::quick_xml::Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_UNNAMED_2),
                            b"atom",
                            false,
                        )?;
                        match self.handle_atom(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                allow_any,
                            } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (
                            ::xsd_parser_types::quick_xml::DeserializerEvent::Continue(event),
                            allow_any_element,
                        );
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (
                            ::xsd_parser_types::quick_xml::DeserializerEvent::Break(event),
                            false,
                        );
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        ) -> ::core::result::Result<
            super::PackageRoundsRoundThemesThemeQuestionsQuestionScenarioElementType,
            ::xsd_parser_types::quick_xml::Error,
        > {
            let state = :: core :: mem :: replace (& mut * self . state__ , PackageRoundsRoundThemesThemeQuestionsQuestionScenarioElementTypeDeserializerState :: Unknown__) ;
            self.finish_state(helper, state)?;
            Ok(
                super::PackageRoundsRoundThemesThemeQuestionsQuestionScenarioElementType {
                    atom: self.atom,
                },
            )
        }
    }
    #[derive(Debug)]
    pub struct PackageRoundsRoundThemesThemeQuestionsQuestionScriptElementTypeDeserializer {
        step: ::std::vec::Vec<super::ParametersType>,
        state__: ::std::boxed::Box<
            PackageRoundsRoundThemesThemeQuestionsQuestionScriptElementTypeDeserializerState,
        >,
    }
    #[derive(Debug)]
    enum PackageRoundsRoundThemesThemeQuestionsQuestionScriptElementTypeDeserializerState {
        Init__ , Step (:: core :: option :: Option << super :: ParametersType as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer >) , Done__ , Unknown__ , }
    impl PackageRoundsRoundThemesThemeQuestionsQuestionScriptElementTypeDeserializer {
        fn from_bytes_start(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            bytes_start: &::xsd_parser_types::quick_xml::BytesStart<'_>,
        ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok (Self { step : :: std :: vec :: Vec :: new () , state__ : :: std :: boxed :: Box :: new (PackageRoundsRoundThemesThemeQuestionsQuestionScriptElementTypeDeserializerState :: Init__) , })
        }
        fn finish_state(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            state: PackageRoundsRoundThemesThemeQuestionsQuestionScriptElementTypeDeserializerState,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            use PackageRoundsRoundThemesThemeQuestionsQuestionScriptElementTypeDeserializerState as S;
            match state {
                S::Step(Some(deserializer)) => self.store_step(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_step(
            &mut self,
            value: super::ParametersType,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            self.step.push(value);
            Ok(())
        }
        fn handle_step<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<'de, super::ParametersType>,
            fallback: &mut ::core::option::Option<
                PackageRoundsRoundThemesThemeQuestionsQuestionScriptElementTypeDeserializerState,
            >,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use PackageRoundsRoundThemesThemeQuestionsQuestionScriptElementTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Step(None));
                *self.state__ = S::Done__;
                return Ok(
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                        event, allow_any,
                    ),
                );
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_step(data)?;
                    *self.state__ = S::Step(None);
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Step(Some(deserializer)));
                    *self.state__ = S::Step(None);
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
    }
    impl<'de>
        ::xsd_parser_types::quick_xml::Deserializer<
            'de,
            super::PackageRoundsRoundThemesThemeQuestionsQuestionScriptElementType,
        > for PackageRoundsRoundThemesThemeQuestionsQuestionScriptElementTypeDeserializer
    {
        fn init(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<
            'de,
            super::PackageRoundsRoundThemesThemeQuestionsQuestionScriptElementType,
        > {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<
            'de,
            super::PackageRoundsRoundThemesThemeQuestionsQuestionScriptElementType,
        > {
            use PackageRoundsRoundThemesThemeQuestionsQuestionScriptElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = ::core::mem::replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Step(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_step(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                allow_any,
                            } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (_, ::xsd_parser_types::quick_xml::Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                            artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(
                                self.finish(helper)?,
                            ),
                            event: ::xsd_parser_types::quick_xml::DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::Step(None);
                        event
                    }
                    (
                        S::Step(None),
                        event @ (::xsd_parser_types::quick_xml::Event::Start(_)
                        | ::xsd_parser_types::quick_xml::Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_UNNAMED_2),
                            b"step",
                            false,
                        )?;
                        match self.handle_step(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                allow_any,
                            } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (
                            ::xsd_parser_types::quick_xml::DeserializerEvent::Continue(event),
                            allow_any_element,
                        );
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (
                            ::xsd_parser_types::quick_xml::DeserializerEvent::Break(event),
                            false,
                        );
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        ) -> ::core::result::Result<
            super::PackageRoundsRoundThemesThemeQuestionsQuestionScriptElementType,
            ::xsd_parser_types::quick_xml::Error,
        > {
            let state = :: core :: mem :: replace (& mut * self . state__ , PackageRoundsRoundThemesThemeQuestionsQuestionScriptElementTypeDeserializerState :: Unknown__) ;
            self.finish_state(helper, state)?;
            Ok(
                super::PackageRoundsRoundThemesThemeQuestionsQuestionScriptElementType {
                    step: self.step,
                },
            )
        }
    }
    #[derive(Debug)]
    pub struct PackageRoundsRoundThemesThemeQuestionsQuestionRightElementTypeDeserializer {
        answer: ::std::vec::Vec<::std::string::String>,
        state__: ::std::boxed::Box<
            PackageRoundsRoundThemesThemeQuestionsQuestionRightElementTypeDeserializerState,
        >,
    }
    #[derive(Debug)]
    enum PackageRoundsRoundThemesThemeQuestionsQuestionRightElementTypeDeserializerState {
        Init__ , Answer (:: core :: option :: Option << :: std :: string :: String as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer >) , Done__ , Unknown__ , }
    impl PackageRoundsRoundThemesThemeQuestionsQuestionRightElementTypeDeserializer {
        fn from_bytes_start(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            bytes_start: &::xsd_parser_types::quick_xml::BytesStart<'_>,
        ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                helper.raise_unexpected_attrib_checked(&attrib)?;
            }
            Ok (Self { answer : :: std :: vec :: Vec :: new () , state__ : :: std :: boxed :: Box :: new (PackageRoundsRoundThemesThemeQuestionsQuestionRightElementTypeDeserializerState :: Init__) , })
        }
        fn finish_state(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            state: PackageRoundsRoundThemesThemeQuestionsQuestionRightElementTypeDeserializerState,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            use PackageRoundsRoundThemesThemeQuestionsQuestionRightElementTypeDeserializerState as S;
            match state {
                S::Answer(Some(deserializer)) => self.store_answer(deserializer.finish(helper)?)?,
                _ => (),
            }
            Ok(())
        }
        fn store_answer(
            &mut self,
            value: ::std::string::String,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            self.answer.push(value);
            Ok(())
        }
        fn handle_answer<'de>(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<'de, ::std::string::String>,
            fallback: &mut ::core::option::Option<
                PackageRoundsRoundThemesThemeQuestionsQuestionRightElementTypeDeserializerState,
            >,
        ) -> ::core::result::Result<
            ::xsd_parser_types::quick_xml::ElementHandlerOutput<'de>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            use PackageRoundsRoundThemesThemeQuestionsQuestionRightElementTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            if artifact.is_none() {
                fallback.get_or_insert(S::Answer(None));
                *self.state__ = S::Done__;
                return Ok(
                    ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                        event, allow_any,
                    ),
                );
            }
            if let Some(fallback) = fallback.take() {
                self.finish_state(helper, fallback)?;
            }
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => unreachable!(),
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_answer(data)?;
                    *self.state__ = S::Answer(None);
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    fallback.get_or_insert(S::Answer(Some(deserializer)));
                    *self.state__ = S::Answer(None);
                    Ok(
                        ::xsd_parser_types::quick_xml::ElementHandlerOutput::from_event(
                            event, allow_any,
                        ),
                    )
                }
            }
        }
    }
    impl<'de>
        ::xsd_parser_types::quick_xml::Deserializer<
            'de,
            super::PackageRoundsRoundThemesThemeQuestionsQuestionRightElementType,
        > for PackageRoundsRoundThemesThemeQuestionsQuestionRightElementTypeDeserializer
    {
        fn init(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<
            'de,
            super::PackageRoundsRoundThemesThemeQuestionsQuestionRightElementType,
        > {
            helper.init_deserializer_from_start_event(event, Self::from_bytes_start)
        }
        fn next(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<
            'de,
            super::PackageRoundsRoundThemesThemeQuestionsQuestionRightElementType,
        > {
            use PackageRoundsRoundThemesThemeQuestionsQuestionRightElementTypeDeserializerState as S;
            let mut event = event;
            let mut fallback = None;
            let mut allow_any_element = false;
            let (event, allow_any) = loop {
                let state = ::core::mem::replace(&mut *self.state__, S::Unknown__);
                event = match (state, event) {
                    (S::Unknown__, _) => unreachable!(),
                    (S::Answer(Some(deserializer)), event) => {
                        let output = deserializer.next(helper, event)?;
                        match self.handle_answer(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                allow_any,
                            } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (_, ::xsd_parser_types::quick_xml::Event::End(_)) => {
                        if let Some(fallback) = fallback.take() {
                            self.finish_state(helper, fallback)?;
                        }
                        return Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                            artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(
                                self.finish(helper)?,
                            ),
                            event: ::xsd_parser_types::quick_xml::DeserializerEvent::None,
                            allow_any: false,
                        });
                    }
                    (S::Init__, event) => {
                        fallback.get_or_insert(S::Init__);
                        *self.state__ = S::Answer(None);
                        event
                    }
                    (
                        S::Answer(None),
                        event @ (::xsd_parser_types::quick_xml::Event::Start(_)
                        | ::xsd_parser_types::quick_xml::Event::Empty(_)),
                    ) => {
                        let output = helper.init_start_tag_deserializer(
                            event,
                            Some(&super::NS_UNNAMED_2),
                            b"answer",
                            false,
                        )?;
                        match self.handle_answer(helper, output, &mut fallback)? {
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Continue {
                                event,
                                allow_any,
                            } => {
                                allow_any_element = allow_any_element || allow_any;
                                event
                            }
                            ::xsd_parser_types::quick_xml::ElementHandlerOutput::Break {
                                event,
                                allow_any,
                            } => break (event, allow_any),
                        }
                    }
                    (S::Done__, event) => {
                        *self.state__ = S::Done__;
                        break (
                            ::xsd_parser_types::quick_xml::DeserializerEvent::Continue(event),
                            allow_any_element,
                        );
                    }
                    (state, event) => {
                        *self.state__ = state;
                        break (
                            ::xsd_parser_types::quick_xml::DeserializerEvent::Break(event),
                            false,
                        );
                    }
                }
            };
            if let Some(fallback) = fallback {
                *self.state__ = fallback;
            }
            Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(self),
                event,
                allow_any,
            })
        }
        fn finish(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        ) -> ::core::result::Result<
            super::PackageRoundsRoundThemesThemeQuestionsQuestionRightElementType,
            ::xsd_parser_types::quick_xml::Error,
        > {
            let state = :: core :: mem :: replace (& mut * self . state__ , PackageRoundsRoundThemesThemeQuestionsQuestionRightElementTypeDeserializerState :: Unknown__) ;
            self.finish_state(helper, state)?;
            Ok(
                super::PackageRoundsRoundThemesThemeQuestionsQuestionRightElementType {
                    answer: self.answer,
                },
            )
        }
    }
    #[derive(Debug)]
    pub struct PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameParamElementTypeDeserializer {
        name: ::std::string::String,
        content: ::core::option::Option<::std::string::String>,
        state__: ::std::boxed::Box<
            PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameParamElementTypeDeserializerState,
        >,
    }
    #[derive(Debug)]
    enum PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameParamElementTypeDeserializerState {
        Init__ , Content__ (< :: std :: string :: String as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer) , Unknown__ , }
    impl PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameParamElementTypeDeserializer {
        fn from_bytes_start(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            bytes_start: &::xsd_parser_types::quick_xml::BytesStart<'_>,
        ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
            let mut name: ::core::option::Option<::std::string::String> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_UNNAMED_2),
                    Some(b"name")
                ) {
                    helper.read_attrib(&mut name, b"name", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok (Self { name : name . ok_or_else (|| :: xsd_parser_types :: quick_xml :: ErrorKind :: MissingAttribute ("name" . into ())) ? , content : None , state__ : :: std :: boxed :: Box :: new (PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameParamElementTypeDeserializerState :: Init__) , })
        }
        fn finish_state(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            state : PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameParamElementTypeDeserializerState,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            if let PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameParamElementTypeDeserializerState :: Content__ (deserializer) = state { self . store_content (deserializer . finish (helper) ?) ? ; }
            Ok(())
        }
        fn store_content(
            &mut self,
            value: ::std::string::String,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            if self.content.is_some() {
                Err(::xsd_parser_types::quick_xml::ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<'de, ::std::string::String>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<
            'de,
            super::PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameParamElementType,
        > {
            use PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameParamElementTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => {
                    Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                        artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::None,
                        event,
                        allow_any,
                    })
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(helper)?;
                    Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                        artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Content__(deserializer);
                    Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                        artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(
                            self,
                        ),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de>
        ::xsd_parser_types::quick_xml::Deserializer<
            'de,
            super::PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameParamElementType,
        > for PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameParamElementTypeDeserializer
    {
        fn init(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<
            'de,
            super::PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameParamElementType,
        > {
            let (::xsd_parser_types::quick_xml::Event::Start(x)
            | ::xsd_parser_types::quick_xml::Event::Empty(x)) = &event
            else {
                return Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                    artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::None,
                    event: ::xsd_parser_types::quick_xml::DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(helper, x)?.next(helper, event)
        }
        fn next(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<
            'de,
            super::PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameParamElementType,
        > {
            use PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameParamElementTypeDeserializerState as S;
            match ::core::mem::replace(&mut *self.state__, S::Unknown__) {
                S::Unknown__ => unreachable!(),
                S::Init__ => {
                    let output =
                        ::xsd_parser_types::quick_xml::ContentDeserializer::init(helper, event)?;
                    self.handle_content(helper, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(helper, event)?;
                    self.handle_content(helper, output)
                }
            }
        }
        fn finish(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        ) -> ::core::result::Result<
            super::PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameParamElementType,
            ::xsd_parser_types::quick_xml::Error,
        > {
            let state = :: core :: mem :: replace (& mut * self . state__ , PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameParamElementTypeDeserializerState :: Unknown__) ;
            self.finish_state(helper, state)?;
            Ok(
                super::PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameParamElementType {
                    name: self.name,
                    content: helper.finish_content(self.content)?,
                },
            )
        }
    }
    #[derive(Debug)]
    pub struct PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomElementTypeDeserializer {
        type_: ::core::option::Option<
            super::PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomType,
        >,
        time: ::core::option::Option<::core::primitive::f64>,
        content: ::core::option::Option<::std::string::String>,
        state__: ::std::boxed::Box<
            PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomElementTypeDeserializerState,
        >,
    }
    #[derive(Debug)]
    enum PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomElementTypeDeserializerState {
        Init__ , Content__ (< :: std :: string :: String as :: xsd_parser_types :: quick_xml :: WithDeserializer > :: Deserializer) , Unknown__ , }
    impl PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomElementTypeDeserializer {
        fn from_bytes_start(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            bytes_start: &::xsd_parser_types::quick_xml::BytesStart<'_>,
        ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
            let mut type_: ::core::option::Option<
                super::PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomType,
            > = None;
            let mut time: ::core::option::Option<::core::primitive::f64> = None;
            for attrib in helper.filter_xmlns_attributes(bytes_start) {
                let attrib = attrib?;
                if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_UNNAMED_2),
                    Some(b"type")
                ) {
                    helper.read_attrib(&mut type_, b"type", &attrib.value)?;
                } else if matches!(
                    helper.resolve_local_name(attrib.key, &super::NS_UNNAMED_2),
                    Some(b"time")
                ) {
                    helper.read_attrib(&mut time, b"time", &attrib.value)?;
                } else {
                    helper.raise_unexpected_attrib_checked(&attrib)?;
                }
            }
            Ok (Self { type_ : type_ , time : time , content : None , state__ : :: std :: boxed :: Box :: new (PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomElementTypeDeserializerState :: Init__) , })
        }
        fn finish_state(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            state : PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomElementTypeDeserializerState,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            if let PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomElementTypeDeserializerState :: Content__ (deserializer) = state { self . store_content (deserializer . finish (helper) ?) ? ; }
            Ok(())
        }
        fn store_content(
            &mut self,
            value: ::std::string::String,
        ) -> ::core::result::Result<(), ::xsd_parser_types::quick_xml::Error> {
            if self.content.is_some() {
                Err(::xsd_parser_types::quick_xml::ErrorKind::DuplicateContent)?;
            }
            self.content = Some(value);
            Ok(())
        }
        fn handle_content<'de>(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            output: ::xsd_parser_types::quick_xml::DeserializerOutput<'de, ::std::string::String>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<
            'de,
            super::PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomElementType,
        > {
            use PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomElementTypeDeserializerState as S;
            let ::xsd_parser_types::quick_xml::DeserializerOutput {
                artifact,
                event,
                allow_any,
            } = output;
            match artifact {
                ::xsd_parser_types::quick_xml::DeserializerArtifact::None => {
                    Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                        artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::None,
                        event,
                        allow_any,
                    })
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data) => {
                    self.store_content(data)?;
                    let data = self.finish(helper)?;
                    Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                        artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Data(data),
                        event,
                        allow_any,
                    })
                }
                ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(deserializer) => {
                    *self.state__ = S::Content__(deserializer);
                    Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                        artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::Deserializer(
                            self,
                        ),
                        event,
                        allow_any,
                    })
                }
            }
        }
    }
    impl<'de>
        ::xsd_parser_types::quick_xml::Deserializer<
            'de,
            super::PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomElementType,
        > for PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomElementTypeDeserializer
    {
        fn init(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<
            'de,
            super::PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomElementType,
        > {
            let (::xsd_parser_types::quick_xml::Event::Start(x)
            | ::xsd_parser_types::quick_xml::Event::Empty(x)) = &event
            else {
                return Ok(::xsd_parser_types::quick_xml::DeserializerOutput {
                    artifact: ::xsd_parser_types::quick_xml::DeserializerArtifact::None,
                    event: ::xsd_parser_types::quick_xml::DeserializerEvent::Break(event),
                    allow_any: false,
                });
            };
            Self::from_bytes_start(helper, x)?.next(helper, event)
        }
        fn next(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            event: ::xsd_parser_types::quick_xml::Event<'de>,
        ) -> ::xsd_parser_types::quick_xml::DeserializerResult<
            'de,
            super::PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomElementType,
        > {
            use PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomElementTypeDeserializerState as S;
            match ::core::mem::replace(&mut *self.state__, S::Unknown__) {
                S::Unknown__ => unreachable!(),
                S::Init__ => {
                    let output =
                        ::xsd_parser_types::quick_xml::ContentDeserializer::init(helper, event)?;
                    self.handle_content(helper, output)
                }
                S::Content__(deserializer) => {
                    let output = deserializer.next(helper, event)?;
                    self.handle_content(helper, output)
                }
            }
        }
        fn finish(
            mut self,
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
        ) -> ::core::result::Result<
            super::PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomElementType,
            ::xsd_parser_types::quick_xml::Error,
        > {
            let state = :: core :: mem :: replace (& mut * self . state__ , PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomElementTypeDeserializerState :: Unknown__) ;
            self.finish_state(helper, state)?;
            Ok(
                super::PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomElementType {
                    type_: self.type_,
                    time: self.time,
                    content: helper.finish_content(self.content)?,
                },
            )
        }
    }
}
pub mod quick_xml_serialize {
    use xsd_parser_types::quick_xml::Serializer as _;
    #[derive(Debug)]
    pub struct InfoTypeSerializer<'ser> {
        pub(super) value: &'ser super::InfoType,
        pub(super) state: ::std::boxed::Box<InfoTypeSerializerState<'ser>>,
        pub(super) name: &'ser ::core::primitive::str,
        pub(super) is_root: ::core::primitive::bool,
    }
    #[derive(Debug)]
    pub(super) enum InfoTypeSerializerState<'ser> {
        Init__,
        Authors(
            ::xsd_parser_types::quick_xml::IterSerializer<
                'ser,
                ::core::option::Option<&'ser super::InfoTypeAuthorsElementType>,
                super::InfoTypeAuthorsElementType,
            >,
        ),
        Sources(
            ::xsd_parser_types::quick_xml::IterSerializer<
                'ser,
                ::core::option::Option<&'ser super::InfoTypeSourcesElementType>,
                super::InfoTypeSourcesElementType,
            >,
        ),
        Comments(
            ::xsd_parser_types::quick_xml::IterSerializer<
                'ser,
                ::core::option::Option<&'ser ::std::string::String>,
                ::std::string::String,
            >,
        ),
        ShowmanComments(
            ::xsd_parser_types::quick_xml::IterSerializer<
                'ser,
                ::core::option::Option<&'ser ::std::string::String>,
                ::std::string::String,
            >,
        ),
        Extension(
            ::xsd_parser_types::quick_xml::IterSerializer<
                'ser,
                ::core::option::Option<&'ser ::std::string::String>,
                ::std::string::String,
            >,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> InfoTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::result::Result<
            ::core::option::Option<::xsd_parser_types::quick_xml::Event<'ser>>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            loop {
                match &mut *self.state {
                    InfoTypeSerializerState::Init__ => {
                        *self.state = InfoTypeSerializerState::Authors(
                            ::xsd_parser_types::quick_xml::IterSerializer::new(
                                self.value.authors.as_ref(),
                                Some("authors"),
                                false,
                            ),
                        );
                        let mut bytes = ::xsd_parser_types::quick_xml::BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        helper.write_xmlns(&mut bytes, None, &super::NS_UNNAMED_2);
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&::xsd_parser_types::misc::NamespacePrefix::XSI),
                                &::xsd_parser_types::misc::Namespace::XSI,
                            );
                        }
                        return Ok(Some(::xsd_parser_types::quick_xml::Event::Start(bytes)));
                    }
                    InfoTypeSerializerState::Authors(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = InfoTypeSerializerState::Sources(
                                ::xsd_parser_types::quick_xml::IterSerializer::new(
                                    self.value.sources.as_ref(),
                                    Some("sources"),
                                    false,
                                ),
                            )
                        }
                    },
                    InfoTypeSerializerState::Sources(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = InfoTypeSerializerState::Comments(
                                ::xsd_parser_types::quick_xml::IterSerializer::new(
                                    self.value.comments.as_ref(),
                                    Some("comments"),
                                    false,
                                ),
                            )
                        }
                    },
                    InfoTypeSerializerState::Comments(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = InfoTypeSerializerState::ShowmanComments(
                                ::xsd_parser_types::quick_xml::IterSerializer::new(
                                    self.value.showman_comments.as_ref(),
                                    Some("showmanComments"),
                                    false,
                                ),
                            )
                        }
                    },
                    InfoTypeSerializerState::ShowmanComments(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = InfoTypeSerializerState::Extension(
                                    ::xsd_parser_types::quick_xml::IterSerializer::new(
                                        self.value.extension.as_ref(),
                                        Some("extension"),
                                        false,
                                    ),
                                )
                            }
                        }
                    }
                    InfoTypeSerializerState::Extension(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = InfoTypeSerializerState::End__,
                    },
                    InfoTypeSerializerState::End__ => {
                        *self.state = InfoTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(::xsd_parser_types::quick_xml::Event::End(
                            ::xsd_parser_types::quick_xml::BytesEnd::new(self.name),
                        )));
                    }
                    InfoTypeSerializerState::Done__ => return Ok(None),
                    InfoTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> ::xsd_parser_types::quick_xml::Serializer<'ser> for InfoTypeSerializer<'ser> {
        fn next(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::option::Option<
            ::core::result::Result<
                ::xsd_parser_types::quick_xml::Event<'ser>,
                ::xsd_parser_types::quick_xml::Error,
            >,
        > {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = InfoTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct PackageElementTypeSerializer<'ser> {
        pub(super) value: &'ser super::PackageElementType,
        pub(super) state: ::std::boxed::Box<PackageElementTypeSerializerState<'ser>>,
        pub(super) name: &'ser ::core::primitive::str,
        pub(super) is_root: ::core::primitive::bool,
    }
    #[derive(Debug)]
    pub(super) enum PackageElementTypeSerializerState<'ser> {
        Init__,
        Tags(
            ::xsd_parser_types::quick_xml::IterSerializer<
                'ser,
                ::core::option::Option<&'ser super::PackageTagsElementType>,
                super::PackageTagsElementType,
            >,
        ),
        Info(
            ::xsd_parser_types::quick_xml::IterSerializer<
                'ser,
                ::core::option::Option<&'ser super::InfoType>,
                super::InfoType,
            >,
        ),
        Global(
            ::xsd_parser_types::quick_xml::IterSerializer<
                'ser,
                ::core::option::Option<&'ser super::PackageGlobalElementType>,
                super::PackageGlobalElementType,
            >,
        ),
        Rounds(
            ::xsd_parser_types::quick_xml::IterSerializer<
                'ser,
                ::core::option::Option<&'ser super::PackageRoundsElementType>,
                super::PackageRoundsElementType,
            >,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> PackageElementTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::result::Result<
            ::core::option::Option<::xsd_parser_types::quick_xml::Event<'ser>>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            loop {
                match &mut *self.state {
                    PackageElementTypeSerializerState::Init__ => {
                        *self.state = PackageElementTypeSerializerState::Tags(
                            ::xsd_parser_types::quick_xml::IterSerializer::new(
                                self.value.tags.as_ref(),
                                Some("tags"),
                                false,
                            ),
                        );
                        let mut bytes = ::xsd_parser_types::quick_xml::BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        helper.write_xmlns(&mut bytes, None, &super::NS_UNNAMED_2);
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&::xsd_parser_types::misc::NamespacePrefix::XSI),
                                &::xsd_parser_types::misc::Namespace::XSI,
                            );
                        }
                        helper.write_attrib_opt(&mut bytes, "id", &self.value.id)?;
                        helper.write_attrib(&mut bytes, "name", &self.value.name)?;
                        helper.write_attrib(&mut bytes, "version", &self.value.version)?;
                        helper.write_attrib_opt(
                            &mut bytes,
                            "restriction",
                            &self.value.restriction,
                        )?;
                        helper.write_attrib_opt(&mut bytes, "date", &self.value.date)?;
                        helper.write_attrib_opt(&mut bytes, "publisher", &self.value.publisher)?;
                        helper.write_attrib_opt(
                            &mut bytes,
                            "difficulty",
                            &self.value.difficulty,
                        )?;
                        helper.write_attrib_opt(&mut bytes, "logo", &self.value.logo)?;
                        helper.write_attrib_opt(&mut bytes, "language", &self.value.language)?;
                        helper.write_attrib_opt(&mut bytes, "generator", &self.value.generator)?;
                        helper.write_attrib_opt(
                            &mut bytes,
                            "contactUri",
                            &self.value.contact_uri,
                        )?;
                        return Ok(Some(::xsd_parser_types::quick_xml::Event::Start(bytes)));
                    }
                    PackageElementTypeSerializerState::Tags(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = PackageElementTypeSerializerState::Info(
                                    ::xsd_parser_types::quick_xml::IterSerializer::new(
                                        self.value.info.as_ref(),
                                        Some("info"),
                                        false,
                                    ),
                                )
                            }
                        }
                    }
                    PackageElementTypeSerializerState::Info(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = PackageElementTypeSerializerState::Global(
                                    ::xsd_parser_types::quick_xml::IterSerializer::new(
                                        self.value.global.as_ref(),
                                        Some("global"),
                                        false,
                                    ),
                                )
                            }
                        }
                    }
                    PackageElementTypeSerializerState::Global(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = PackageElementTypeSerializerState::Rounds(
                                    ::xsd_parser_types::quick_xml::IterSerializer::new(
                                        self.value.rounds.as_ref(),
                                        Some("rounds"),
                                        false,
                                    ),
                                )
                            }
                        }
                    }
                    PackageElementTypeSerializerState::Rounds(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = PackageElementTypeSerializerState::End__,
                        }
                    }
                    PackageElementTypeSerializerState::End__ => {
                        *self.state = PackageElementTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(::xsd_parser_types::quick_xml::Event::End(
                            ::xsd_parser_types::quick_xml::BytesEnd::new(self.name),
                        )));
                    }
                    PackageElementTypeSerializerState::Done__ => return Ok(None),
                    PackageElementTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> ::xsd_parser_types::quick_xml::Serializer<'ser> for PackageElementTypeSerializer<'ser> {
        fn next(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::option::Option<
            ::core::result::Result<
                ::xsd_parser_types::quick_xml::Event<'ser>,
                ::xsd_parser_types::quick_xml::Error,
            >,
        > {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = PackageElementTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ParameterTypeSerializer<'ser> {
        pub(super) value: &'ser super::ParameterType,
        pub(super) state: ::std::boxed::Box<ParameterTypeSerializerState<'ser>>,
        pub(super) name: &'ser ::core::primitive::str,
        pub(super) is_root: ::core::primitive::bool,
    }
    #[derive(Debug)]
    pub(super) enum ParameterTypeSerializerState<'ser> {
        Init__,
        TextBefore(
            ::xsd_parser_types::quick_xml::IterSerializer<
                'ser,
                ::core::option::Option<&'ser ::xsd_parser_types::xml::Text>,
                ::xsd_parser_types::xml::Text,
            >,
        ),
        Item(
            ::xsd_parser_types::quick_xml::IterSerializer<
                'ser,
                &'ser [::xsd_parser_types::xml::Mixed<super::ParameterTypeItemElementType>],
                ::xsd_parser_types::xml::Mixed<super::ParameterTypeItemElementType>,
            >,
        ),
        Param(
            ::xsd_parser_types::quick_xml::IterSerializer<
                'ser,
                &'ser [::xsd_parser_types::xml::Mixed<super::ParameterType>],
                ::xsd_parser_types::xml::Mixed<super::ParameterType>,
            >,
        ),
        NumberSet(
            ::xsd_parser_types::quick_xml::IterSerializer<
                'ser,
                ::core::option::Option<
                    &'ser ::xsd_parser_types::xml::Mixed<super::ParameterTypeNumberSetElementType>,
                >,
                ::xsd_parser_types::xml::Mixed<super::ParameterTypeNumberSetElementType>,
            >,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ParameterTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::result::Result<
            ::core::option::Option<::xsd_parser_types::quick_xml::Event<'ser>>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            loop {
                match &mut *self.state {
                    ParameterTypeSerializerState::Init__ => {
                        *self.state = ParameterTypeSerializerState::TextBefore(
                            ::xsd_parser_types::quick_xml::IterSerializer::new(
                                self.value.text_before.as_ref(),
                                Some(""),
                                false,
                            ),
                        );
                        let mut bytes = ::xsd_parser_types::quick_xml::BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        helper.write_xmlns(&mut bytes, None, &super::NS_UNNAMED_2);
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&::xsd_parser_types::misc::NamespacePrefix::XSI),
                                &::xsd_parser_types::misc::Namespace::XSI,
                            );
                        }
                        helper.write_attrib_opt(&mut bytes, "name", &self.value.name)?;
                        helper.write_attrib_opt(&mut bytes, "type", &self.value.type_)?;
                        return Ok(Some(::xsd_parser_types::quick_xml::Event::Start(bytes)));
                    }
                    ParameterTypeSerializerState::TextBefore(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = ParameterTypeSerializerState::Item(
                                    ::xsd_parser_types::quick_xml::IterSerializer::new(
                                        &self.value.item[..],
                                        Some("item"),
                                        false,
                                    ),
                                )
                            }
                        }
                    }
                    ParameterTypeSerializerState::Item(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = ParameterTypeSerializerState::Param(
                                ::xsd_parser_types::quick_xml::IterSerializer::new(
                                    &self.value.param[..],
                                    Some("param"),
                                    false,
                                ),
                            )
                        }
                    },
                    ParameterTypeSerializerState::Param(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => {
                            *self.state = ParameterTypeSerializerState::NumberSet(
                                ::xsd_parser_types::quick_xml::IterSerializer::new(
                                    self.value.number_set.as_ref(),
                                    Some("numberSet"),
                                    false,
                                ),
                            )
                        }
                    },
                    ParameterTypeSerializerState::NumberSet(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = ParameterTypeSerializerState::End__,
                        }
                    }
                    ParameterTypeSerializerState::End__ => {
                        *self.state = ParameterTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(::xsd_parser_types::quick_xml::Event::End(
                            ::xsd_parser_types::quick_xml::BytesEnd::new(self.name),
                        )));
                    }
                    ParameterTypeSerializerState::Done__ => return Ok(None),
                    ParameterTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> ::xsd_parser_types::quick_xml::Serializer<'ser> for ParameterTypeSerializer<'ser> {
        fn next(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::option::Option<
            ::core::result::Result<
                ::xsd_parser_types::quick_xml::Event<'ser>,
                ::xsd_parser_types::quick_xml::Error,
            >,
        > {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ParameterTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ParametersTypeSerializer<'ser> {
        pub(super) value: &'ser super::ParametersType,
        pub(super) state: ::std::boxed::Box<ParametersTypeSerializerState<'ser>>,
        pub(super) name: &'ser ::core::primitive::str,
        pub(super) is_root: ::core::primitive::bool,
    }
    #[derive(Debug)]
    pub(super) enum ParametersTypeSerializerState<'ser> {
        Init__,
        Param(
            ::xsd_parser_types::quick_xml::IterSerializer<
                'ser,
                &'ser [super::ParameterType],
                super::ParameterType,
            >,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ParametersTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::result::Result<
            ::core::option::Option<::xsd_parser_types::quick_xml::Event<'ser>>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            loop {
                match &mut *self.state {
                    ParametersTypeSerializerState::Init__ => {
                        *self.state = ParametersTypeSerializerState::Param(
                            ::xsd_parser_types::quick_xml::IterSerializer::new(
                                &self.value.param[..],
                                Some("param"),
                                false,
                            ),
                        );
                        let mut bytes = ::xsd_parser_types::quick_xml::BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        helper.write_xmlns(&mut bytes, None, &super::NS_UNNAMED_2);
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&::xsd_parser_types::misc::NamespacePrefix::XSI),
                                &::xsd_parser_types::misc::Namespace::XSI,
                            );
                        }
                        return Ok(Some(::xsd_parser_types::quick_xml::Event::Start(bytes)));
                    }
                    ParametersTypeSerializerState::Param(x) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = ParametersTypeSerializerState::End__,
                    },
                    ParametersTypeSerializerState::End__ => {
                        *self.state = ParametersTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(::xsd_parser_types::quick_xml::Event::End(
                            ::xsd_parser_types::quick_xml::BytesEnd::new(self.name),
                        )));
                    }
                    ParametersTypeSerializerState::Done__ => return Ok(None),
                    ParametersTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> ::xsd_parser_types::quick_xml::Serializer<'ser> for ParametersTypeSerializer<'ser> {
        fn next(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::option::Option<
            ::core::result::Result<
                ::xsd_parser_types::quick_xml::Event<'ser>,
                ::xsd_parser_types::quick_xml::Error,
            >,
        > {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ParametersTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct InfoTypeAuthorsElementTypeSerializer<'ser> {
        pub(super) value: &'ser super::InfoTypeAuthorsElementType,
        pub(super) state: ::std::boxed::Box<InfoTypeAuthorsElementTypeSerializerState<'ser>>,
        pub(super) name: &'ser ::core::primitive::str,
        pub(super) is_root: ::core::primitive::bool,
    }
    #[derive(Debug)]
    pub(super) enum InfoTypeAuthorsElementTypeSerializerState<'ser> {
        Init__,
        Author(
            ::xsd_parser_types::quick_xml::IterSerializer<
                'ser,
                &'ser [::std::string::String],
                ::std::string::String,
            >,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> InfoTypeAuthorsElementTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::result::Result<
            ::core::option::Option<::xsd_parser_types::quick_xml::Event<'ser>>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            loop {
                match &mut *self.state {
                    InfoTypeAuthorsElementTypeSerializerState::Init__ => {
                        *self.state = InfoTypeAuthorsElementTypeSerializerState::Author(
                            ::xsd_parser_types::quick_xml::IterSerializer::new(
                                &self.value.author[..],
                                Some("author"),
                                false,
                            ),
                        );
                        let mut bytes = ::xsd_parser_types::quick_xml::BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        helper.write_xmlns(&mut bytes, None, &super::NS_UNNAMED_2);
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&::xsd_parser_types::misc::NamespacePrefix::XSI),
                                &::xsd_parser_types::misc::Namespace::XSI,
                            );
                        }
                        return Ok(Some(::xsd_parser_types::quick_xml::Event::Start(bytes)));
                    }
                    InfoTypeAuthorsElementTypeSerializerState::Author(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = InfoTypeAuthorsElementTypeSerializerState::End__,
                        }
                    }
                    InfoTypeAuthorsElementTypeSerializerState::End__ => {
                        *self.state = InfoTypeAuthorsElementTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(::xsd_parser_types::quick_xml::Event::End(
                            ::xsd_parser_types::quick_xml::BytesEnd::new(self.name),
                        )));
                    }
                    InfoTypeAuthorsElementTypeSerializerState::Done__ => return Ok(None),
                    InfoTypeAuthorsElementTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> ::xsd_parser_types::quick_xml::Serializer<'ser>
        for InfoTypeAuthorsElementTypeSerializer<'ser>
    {
        fn next(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::option::Option<
            ::core::result::Result<
                ::xsd_parser_types::quick_xml::Event<'ser>,
                ::xsd_parser_types::quick_xml::Error,
            >,
        > {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = InfoTypeAuthorsElementTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct InfoTypeSourcesElementTypeSerializer<'ser> {
        pub(super) value: &'ser super::InfoTypeSourcesElementType,
        pub(super) state: ::std::boxed::Box<InfoTypeSourcesElementTypeSerializerState<'ser>>,
        pub(super) name: &'ser ::core::primitive::str,
        pub(super) is_root: ::core::primitive::bool,
    }
    #[derive(Debug)]
    pub(super) enum InfoTypeSourcesElementTypeSerializerState<'ser> {
        Init__,
        Source(
            ::xsd_parser_types::quick_xml::IterSerializer<
                'ser,
                &'ser [::std::string::String],
                ::std::string::String,
            >,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> InfoTypeSourcesElementTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::result::Result<
            ::core::option::Option<::xsd_parser_types::quick_xml::Event<'ser>>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            loop {
                match &mut *self.state {
                    InfoTypeSourcesElementTypeSerializerState::Init__ => {
                        *self.state = InfoTypeSourcesElementTypeSerializerState::Source(
                            ::xsd_parser_types::quick_xml::IterSerializer::new(
                                &self.value.source[..],
                                Some("source"),
                                false,
                            ),
                        );
                        let mut bytes = ::xsd_parser_types::quick_xml::BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        helper.write_xmlns(&mut bytes, None, &super::NS_UNNAMED_2);
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&::xsd_parser_types::misc::NamespacePrefix::XSI),
                                &::xsd_parser_types::misc::Namespace::XSI,
                            );
                        }
                        return Ok(Some(::xsd_parser_types::quick_xml::Event::Start(bytes)));
                    }
                    InfoTypeSourcesElementTypeSerializerState::Source(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = InfoTypeSourcesElementTypeSerializerState::End__,
                        }
                    }
                    InfoTypeSourcesElementTypeSerializerState::End__ => {
                        *self.state = InfoTypeSourcesElementTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(::xsd_parser_types::quick_xml::Event::End(
                            ::xsd_parser_types::quick_xml::BytesEnd::new(self.name),
                        )));
                    }
                    InfoTypeSourcesElementTypeSerializerState::Done__ => return Ok(None),
                    InfoTypeSourcesElementTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> ::xsd_parser_types::quick_xml::Serializer<'ser>
        for InfoTypeSourcesElementTypeSerializer<'ser>
    {
        fn next(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::option::Option<
            ::core::result::Result<
                ::xsd_parser_types::quick_xml::Event<'ser>,
                ::xsd_parser_types::quick_xml::Error,
            >,
        > {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = InfoTypeSourcesElementTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct PackageTagsElementTypeSerializer<'ser> {
        pub(super) value: &'ser super::PackageTagsElementType,
        pub(super) state: ::std::boxed::Box<PackageTagsElementTypeSerializerState<'ser>>,
        pub(super) name: &'ser ::core::primitive::str,
        pub(super) is_root: ::core::primitive::bool,
    }
    #[derive(Debug)]
    pub(super) enum PackageTagsElementTypeSerializerState<'ser> {
        Init__,
        Tag(
            ::xsd_parser_types::quick_xml::IterSerializer<
                'ser,
                &'ser [::std::string::String],
                ::std::string::String,
            >,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> PackageTagsElementTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::result::Result<
            ::core::option::Option<::xsd_parser_types::quick_xml::Event<'ser>>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            loop {
                match &mut *self.state {
                    PackageTagsElementTypeSerializerState::Init__ => {
                        *self.state = PackageTagsElementTypeSerializerState::Tag(
                            ::xsd_parser_types::quick_xml::IterSerializer::new(
                                &self.value.tag[..],
                                Some("tag"),
                                false,
                            ),
                        );
                        let mut bytes = ::xsd_parser_types::quick_xml::BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        helper.write_xmlns(&mut bytes, None, &super::NS_UNNAMED_2);
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&::xsd_parser_types::misc::NamespacePrefix::XSI),
                                &::xsd_parser_types::misc::Namespace::XSI,
                            );
                        }
                        return Ok(Some(::xsd_parser_types::quick_xml::Event::Start(bytes)));
                    }
                    PackageTagsElementTypeSerializerState::Tag(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = PackageTagsElementTypeSerializerState::End__,
                        }
                    }
                    PackageTagsElementTypeSerializerState::End__ => {
                        *self.state = PackageTagsElementTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(::xsd_parser_types::quick_xml::Event::End(
                            ::xsd_parser_types::quick_xml::BytesEnd::new(self.name),
                        )));
                    }
                    PackageTagsElementTypeSerializerState::Done__ => return Ok(None),
                    PackageTagsElementTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> ::xsd_parser_types::quick_xml::Serializer<'ser>
        for PackageTagsElementTypeSerializer<'ser>
    {
        fn next(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::option::Option<
            ::core::result::Result<
                ::xsd_parser_types::quick_xml::Event<'ser>,
                ::xsd_parser_types::quick_xml::Error,
            >,
        > {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = PackageTagsElementTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct PackageGlobalElementTypeSerializer<'ser> {
        pub(super) value: &'ser super::PackageGlobalElementType,
        pub(super) state: ::std::boxed::Box<PackageGlobalElementTypeSerializerState<'ser>>,
        pub(super) name: &'ser ::core::primitive::str,
        pub(super) is_root: ::core::primitive::bool,
    }
    #[derive(Debug)]
    pub(super) enum PackageGlobalElementTypeSerializerState<'ser> {
        Init__ , Authors (< super :: PackageGlobalAuthorsElementType as :: xsd_parser_types :: quick_xml :: WithSerializer > :: Serializer < 'ser >) , Sources (< super :: PackageGlobalSourcesElementType as :: xsd_parser_types :: quick_xml :: WithSerializer > :: Serializer < 'ser >) , End__ , Done__ , Phantom__ (& 'ser ()) , }
    impl<'ser> PackageGlobalElementTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::result::Result<
            ::core::option::Option<::xsd_parser_types::quick_xml::Event<'ser>>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            loop {
                match &mut *self.state {
                    PackageGlobalElementTypeSerializerState::Init__ => {
                        *self.state = PackageGlobalElementTypeSerializerState::Authors(
                            ::xsd_parser_types::quick_xml::WithSerializer::serializer(
                                &self.value.authors,
                                Some("Authors"),
                                false,
                            )?,
                        );
                        let mut bytes = ::xsd_parser_types::quick_xml::BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        helper.write_xmlns(&mut bytes, None, &super::NS_UNNAMED_2);
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&::xsd_parser_types::misc::NamespacePrefix::XSI),
                                &::xsd_parser_types::misc::Namespace::XSI,
                            );
                        }
                        return Ok(Some(::xsd_parser_types::quick_xml::Event::Start(bytes)));
                    }
                    PackageGlobalElementTypeSerializerState::Authors(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = PackageGlobalElementTypeSerializerState::Sources(
                                    ::xsd_parser_types::quick_xml::WithSerializer::serializer(
                                        &self.value.sources,
                                        Some("Sources"),
                                        false,
                                    )?,
                                )
                            }
                        }
                    }
                    PackageGlobalElementTypeSerializerState::Sources(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = PackageGlobalElementTypeSerializerState::End__,
                        }
                    }
                    PackageGlobalElementTypeSerializerState::End__ => {
                        *self.state = PackageGlobalElementTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(::xsd_parser_types::quick_xml::Event::End(
                            ::xsd_parser_types::quick_xml::BytesEnd::new(self.name),
                        )));
                    }
                    PackageGlobalElementTypeSerializerState::Done__ => return Ok(None),
                    PackageGlobalElementTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> ::xsd_parser_types::quick_xml::Serializer<'ser>
        for PackageGlobalElementTypeSerializer<'ser>
    {
        fn next(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::option::Option<
            ::core::result::Result<
                ::xsd_parser_types::quick_xml::Event<'ser>,
                ::xsd_parser_types::quick_xml::Error,
            >,
        > {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = PackageGlobalElementTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct PackageRoundsElementTypeSerializer<'ser> {
        pub(super) value: &'ser super::PackageRoundsElementType,
        pub(super) state: ::std::boxed::Box<PackageRoundsElementTypeSerializerState<'ser>>,
        pub(super) name: &'ser ::core::primitive::str,
        pub(super) is_root: ::core::primitive::bool,
    }
    #[derive(Debug)]
    pub(super) enum PackageRoundsElementTypeSerializerState<'ser> {
        Init__,
        Round(
            ::xsd_parser_types::quick_xml::IterSerializer<
                'ser,
                &'ser [super::PackageRoundsRoundElementType],
                super::PackageRoundsRoundElementType,
            >,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> PackageRoundsElementTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::result::Result<
            ::core::option::Option<::xsd_parser_types::quick_xml::Event<'ser>>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            loop {
                match &mut *self.state {
                    PackageRoundsElementTypeSerializerState::Init__ => {
                        *self.state = PackageRoundsElementTypeSerializerState::Round(
                            ::xsd_parser_types::quick_xml::IterSerializer::new(
                                &self.value.round[..],
                                Some("round"),
                                false,
                            ),
                        );
                        let mut bytes = ::xsd_parser_types::quick_xml::BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        helper.write_xmlns(&mut bytes, None, &super::NS_UNNAMED_2);
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&::xsd_parser_types::misc::NamespacePrefix::XSI),
                                &::xsd_parser_types::misc::Namespace::XSI,
                            );
                        }
                        return Ok(Some(::xsd_parser_types::quick_xml::Event::Start(bytes)));
                    }
                    PackageRoundsElementTypeSerializerState::Round(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state = PackageRoundsElementTypeSerializerState::End__,
                        }
                    }
                    PackageRoundsElementTypeSerializerState::End__ => {
                        *self.state = PackageRoundsElementTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(::xsd_parser_types::quick_xml::Event::End(
                            ::xsd_parser_types::quick_xml::BytesEnd::new(self.name),
                        )));
                    }
                    PackageRoundsElementTypeSerializerState::Done__ => return Ok(None),
                    PackageRoundsElementTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> ::xsd_parser_types::quick_xml::Serializer<'ser>
        for PackageRoundsElementTypeSerializer<'ser>
    {
        fn next(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::option::Option<
            ::core::result::Result<
                ::xsd_parser_types::quick_xml::Event<'ser>,
                ::xsd_parser_types::quick_xml::Error,
            >,
        > {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = PackageRoundsElementTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ParameterTypeItemElementTypeSerializer<'ser> {
        pub(super) value: &'ser super::ParameterTypeItemElementType,
        pub(super) state: ::std::boxed::Box<ParameterTypeItemElementTypeSerializerState<'ser>>,
        pub(super) name: &'ser ::core::primitive::str,
        pub(super) is_root: ::core::primitive::bool,
    }
    #[derive(Debug)]
    pub(super) enum ParameterTypeItemElementTypeSerializerState<'ser> {
        Init__,
        Content__(
            <::std::string::String as ::xsd_parser_types::quick_xml::WithSerializer>::Serializer<
                'ser,
            >,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ParameterTypeItemElementTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::result::Result<
            ::core::option::Option<::xsd_parser_types::quick_xml::Event<'ser>>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            loop {
                match &mut *self.state {
                    ParameterTypeItemElementTypeSerializerState::Init__ => {
                        *self.state = ParameterTypeItemElementTypeSerializerState::Content__(
                            ::xsd_parser_types::quick_xml::WithSerializer::serializer(
                                &self.value.content,
                                None,
                                false,
                            )?,
                        );
                        let mut bytes = ::xsd_parser_types::quick_xml::BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        helper.write_xmlns(&mut bytes, None, &super::NS_UNNAMED_2);
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&::xsd_parser_types::misc::NamespacePrefix::XSI),
                                &::xsd_parser_types::misc::Namespace::XSI,
                            );
                        }
                        helper.write_attrib_opt(&mut bytes, "type", &self.value.type_)?;
                        helper.write_attrib_opt(&mut bytes, "isRef", &self.value.is_ref)?;
                        helper.write_attrib_opt(&mut bytes, "placement", &self.value.placement)?;
                        helper.write_attrib_opt(&mut bytes, "duration", &self.value.duration)?;
                        helper.write_attrib_opt(
                            &mut bytes,
                            "waitForFinish",
                            &self.value.wait_for_finish,
                        )?;
                        return Ok(Some(::xsd_parser_types::quick_xml::Event::Start(bytes)));
                    }
                    ParameterTypeItemElementTypeSerializerState::Content__(x) => match x
                        .next(helper)
                        .transpose()?
                    {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = ParameterTypeItemElementTypeSerializerState::End__,
                    },
                    ParameterTypeItemElementTypeSerializerState::End__ => {
                        *self.state = ParameterTypeItemElementTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(::xsd_parser_types::quick_xml::Event::End(
                            ::xsd_parser_types::quick_xml::BytesEnd::new(self.name),
                        )));
                    }
                    ParameterTypeItemElementTypeSerializerState::Done__ => return Ok(None),
                    ParameterTypeItemElementTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> ::xsd_parser_types::quick_xml::Serializer<'ser>
        for ParameterTypeItemElementTypeSerializer<'ser>
    {
        fn next(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::option::Option<
            ::core::result::Result<
                ::xsd_parser_types::quick_xml::Event<'ser>,
                ::xsd_parser_types::quick_xml::Error,
            >,
        > {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ParameterTypeItemElementTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct ParameterTypeNumberSetElementTypeSerializer<'ser> {
        pub(super) value: &'ser super::ParameterTypeNumberSetElementType,
        pub(super) state: ::std::boxed::Box<ParameterTypeNumberSetElementTypeSerializerState<'ser>>,
        pub(super) name: &'ser ::core::primitive::str,
        pub(super) is_root: ::core::primitive::bool,
    }
    #[derive(Debug)]
    pub(super) enum ParameterTypeNumberSetElementTypeSerializerState<'ser> {
        Init__,
        Content__(
            <::std::string::String as ::xsd_parser_types::quick_xml::WithSerializer>::Serializer<
                'ser,
            >,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> ParameterTypeNumberSetElementTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::result::Result<
            ::core::option::Option<::xsd_parser_types::quick_xml::Event<'ser>>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            loop {
                match &mut *self.state {
                    ParameterTypeNumberSetElementTypeSerializerState::Init__ => {
                        *self.state = ParameterTypeNumberSetElementTypeSerializerState::Content__(
                            ::xsd_parser_types::quick_xml::WithSerializer::serializer(
                                &self.value.content,
                                None,
                                false,
                            )?,
                        );
                        let mut bytes = ::xsd_parser_types::quick_xml::BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        helper.write_xmlns(&mut bytes, None, &super::NS_UNNAMED_2);
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&::xsd_parser_types::misc::NamespacePrefix::XSI),
                                &::xsd_parser_types::misc::Namespace::XSI,
                            );
                        }
                        helper.write_attrib_opt(&mut bytes, "minimum", &self.value.minimum)?;
                        helper.write_attrib_opt(&mut bytes, "maximum", &self.value.maximum)?;
                        helper.write_attrib_opt(&mut bytes, "step", &self.value.step)?;
                        return Ok(Some(::xsd_parser_types::quick_xml::Event::Start(bytes)));
                    }
                    ParameterTypeNumberSetElementTypeSerializerState::Content__(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state =
                                    ParameterTypeNumberSetElementTypeSerializerState::End__
                            }
                        }
                    }
                    ParameterTypeNumberSetElementTypeSerializerState::End__ => {
                        *self.state = ParameterTypeNumberSetElementTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(::xsd_parser_types::quick_xml::Event::End(
                            ::xsd_parser_types::quick_xml::BytesEnd::new(self.name),
                        )));
                    }
                    ParameterTypeNumberSetElementTypeSerializerState::Done__ => return Ok(None),
                    ParameterTypeNumberSetElementTypeSerializerState::Phantom__(_) => {
                        unreachable!()
                    }
                }
            }
        }
    }
    impl<'ser> ::xsd_parser_types::quick_xml::Serializer<'ser>
        for ParameterTypeNumberSetElementTypeSerializer<'ser>
    {
        fn next(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::option::Option<
            ::core::result::Result<
                ::xsd_parser_types::quick_xml::Event<'ser>,
                ::xsd_parser_types::quick_xml::Error,
            >,
        > {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = ParameterTypeNumberSetElementTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct PackageGlobalAuthorsElementTypeSerializer<'ser> {
        pub(super) value: &'ser super::PackageGlobalAuthorsElementType,
        pub(super) state: ::std::boxed::Box<PackageGlobalAuthorsElementTypeSerializerState<'ser>>,
        pub(super) name: &'ser ::core::primitive::str,
        pub(super) is_root: ::core::primitive::bool,
    }
    #[derive(Debug)]
    pub(super) enum PackageGlobalAuthorsElementTypeSerializerState<'ser> {
        Init__,
        Name(
            <::std::string::String as ::xsd_parser_types::quick_xml::WithSerializer>::Serializer<
                'ser,
            >,
        ),
        SecondName(
            <::std::string::String as ::xsd_parser_types::quick_xml::WithSerializer>::Serializer<
                'ser,
            >,
        ),
        Surname(
            <::std::string::String as ::xsd_parser_types::quick_xml::WithSerializer>::Serializer<
                'ser,
            >,
        ),
        Country(
            <::std::string::String as ::xsd_parser_types::quick_xml::WithSerializer>::Serializer<
                'ser,
            >,
        ),
        City(
            <::std::string::String as ::xsd_parser_types::quick_xml::WithSerializer>::Serializer<
                'ser,
            >,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> PackageGlobalAuthorsElementTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::result::Result<
            ::core::option::Option<::xsd_parser_types::quick_xml::Event<'ser>>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            loop {
                match &mut *self.state {
                    PackageGlobalAuthorsElementTypeSerializerState::Init__ => {
                        *self.state = PackageGlobalAuthorsElementTypeSerializerState::Name(
                            ::xsd_parser_types::quick_xml::WithSerializer::serializer(
                                &self.value.name,
                                Some("Name"),
                                false,
                            )?,
                        );
                        let mut bytes = ::xsd_parser_types::quick_xml::BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        helper.write_xmlns(&mut bytes, None, &super::NS_UNNAMED_2);
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&::xsd_parser_types::misc::NamespacePrefix::XSI),
                                &::xsd_parser_types::misc::Namespace::XSI,
                            );
                        }
                        helper.write_attrib(&mut bytes, "id", &self.value.id)?;
                        return Ok(Some(::xsd_parser_types::quick_xml::Event::Start(bytes)));
                    }
                    PackageGlobalAuthorsElementTypeSerializerState::Name(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state =
                                    PackageGlobalAuthorsElementTypeSerializerState::SecondName(
                                        ::xsd_parser_types::quick_xml::WithSerializer::serializer(
                                            &self.value.second_name,
                                            Some("SecondName"),
                                            false,
                                        )?,
                                    )
                            }
                        }
                    }
                    PackageGlobalAuthorsElementTypeSerializerState::SecondName(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state =
                                    PackageGlobalAuthorsElementTypeSerializerState::Surname(
                                        ::xsd_parser_types::quick_xml::WithSerializer::serializer(
                                            &self.value.surname,
                                            Some("Surname"),
                                            false,
                                        )?,
                                    )
                            }
                        }
                    }
                    PackageGlobalAuthorsElementTypeSerializerState::Surname(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state =
                                    PackageGlobalAuthorsElementTypeSerializerState::Country(
                                        ::xsd_parser_types::quick_xml::WithSerializer::serializer(
                                            &self.value.country,
                                            Some("Country"),
                                            false,
                                        )?,
                                    )
                            }
                        }
                    }
                    PackageGlobalAuthorsElementTypeSerializerState::Country(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = PackageGlobalAuthorsElementTypeSerializerState::City(
                                    ::xsd_parser_types::quick_xml::WithSerializer::serializer(
                                        &self.value.city,
                                        Some("City"),
                                        false,
                                    )?,
                                )
                            }
                        }
                    }
                    PackageGlobalAuthorsElementTypeSerializerState::City(x) => match x
                        .next(helper)
                        .transpose()?
                    {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = PackageGlobalAuthorsElementTypeSerializerState::End__,
                    },
                    PackageGlobalAuthorsElementTypeSerializerState::End__ => {
                        *self.state = PackageGlobalAuthorsElementTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(::xsd_parser_types::quick_xml::Event::End(
                            ::xsd_parser_types::quick_xml::BytesEnd::new(self.name),
                        )));
                    }
                    PackageGlobalAuthorsElementTypeSerializerState::Done__ => return Ok(None),
                    PackageGlobalAuthorsElementTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> ::xsd_parser_types::quick_xml::Serializer<'ser>
        for PackageGlobalAuthorsElementTypeSerializer<'ser>
    {
        fn next(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::option::Option<
            ::core::result::Result<
                ::xsd_parser_types::quick_xml::Event<'ser>,
                ::xsd_parser_types::quick_xml::Error,
            >,
        > {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = PackageGlobalAuthorsElementTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct PackageGlobalSourcesElementTypeSerializer<'ser> {
        pub(super) value: &'ser super::PackageGlobalSourcesElementType,
        pub(super) state: ::std::boxed::Box<PackageGlobalSourcesElementTypeSerializerState<'ser>>,
        pub(super) name: &'ser ::core::primitive::str,
        pub(super) is_root: ::core::primitive::bool,
    }
    #[derive(Debug)]
    pub(super) enum PackageGlobalSourcesElementTypeSerializerState<'ser> {
        Init__,
        Author(
            <::std::string::String as ::xsd_parser_types::quick_xml::WithSerializer>::Serializer<
                'ser,
            >,
        ),
        Title(
            <::std::string::String as ::xsd_parser_types::quick_xml::WithSerializer>::Serializer<
                'ser,
            >,
        ),
        Year(
            <::std::string::String as ::xsd_parser_types::quick_xml::WithSerializer>::Serializer<
                'ser,
            >,
        ),
        Publish(
            <::std::string::String as ::xsd_parser_types::quick_xml::WithSerializer>::Serializer<
                'ser,
            >,
        ),
        City(
            <::std::string::String as ::xsd_parser_types::quick_xml::WithSerializer>::Serializer<
                'ser,
            >,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> PackageGlobalSourcesElementTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::result::Result<
            ::core::option::Option<::xsd_parser_types::quick_xml::Event<'ser>>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            loop {
                match &mut *self.state {
                    PackageGlobalSourcesElementTypeSerializerState::Init__ => {
                        *self.state = PackageGlobalSourcesElementTypeSerializerState::Author(
                            ::xsd_parser_types::quick_xml::WithSerializer::serializer(
                                &self.value.author,
                                Some("Author"),
                                false,
                            )?,
                        );
                        let mut bytes = ::xsd_parser_types::quick_xml::BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        helper.write_xmlns(&mut bytes, None, &super::NS_UNNAMED_2);
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&::xsd_parser_types::misc::NamespacePrefix::XSI),
                                &::xsd_parser_types::misc::Namespace::XSI,
                            );
                        }
                        helper.write_attrib(&mut bytes, "id", &self.value.id)?;
                        return Ok(Some(::xsd_parser_types::quick_xml::Event::Start(bytes)));
                    }
                    PackageGlobalSourcesElementTypeSerializerState::Author(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = PackageGlobalSourcesElementTypeSerializerState::Title(
                                    ::xsd_parser_types::quick_xml::WithSerializer::serializer(
                                        &self.value.title,
                                        Some("Title"),
                                        false,
                                    )?,
                                )
                            }
                        }
                    }
                    PackageGlobalSourcesElementTypeSerializerState::Title(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = PackageGlobalSourcesElementTypeSerializerState::Year(
                                    ::xsd_parser_types::quick_xml::WithSerializer::serializer(
                                        &self.value.year,
                                        Some("Year"),
                                        false,
                                    )?,
                                )
                            }
                        }
                    }
                    PackageGlobalSourcesElementTypeSerializerState::Year(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state =
                                    PackageGlobalSourcesElementTypeSerializerState::Publish(
                                        ::xsd_parser_types::quick_xml::WithSerializer::serializer(
                                            &self.value.publish,
                                            Some("Publish"),
                                            false,
                                        )?,
                                    )
                            }
                        }
                    }
                    PackageGlobalSourcesElementTypeSerializerState::Publish(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = PackageGlobalSourcesElementTypeSerializerState::City(
                                    ::xsd_parser_types::quick_xml::WithSerializer::serializer(
                                        &self.value.city,
                                        Some("City"),
                                        false,
                                    )?,
                                )
                            }
                        }
                    }
                    PackageGlobalSourcesElementTypeSerializerState::City(x) => match x
                        .next(helper)
                        .transpose()?
                    {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = PackageGlobalSourcesElementTypeSerializerState::End__,
                    },
                    PackageGlobalSourcesElementTypeSerializerState::End__ => {
                        *self.state = PackageGlobalSourcesElementTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(::xsd_parser_types::quick_xml::Event::End(
                            ::xsd_parser_types::quick_xml::BytesEnd::new(self.name),
                        )));
                    }
                    PackageGlobalSourcesElementTypeSerializerState::Done__ => return Ok(None),
                    PackageGlobalSourcesElementTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> ::xsd_parser_types::quick_xml::Serializer<'ser>
        for PackageGlobalSourcesElementTypeSerializer<'ser>
    {
        fn next(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::option::Option<
            ::core::result::Result<
                ::xsd_parser_types::quick_xml::Event<'ser>,
                ::xsd_parser_types::quick_xml::Error,
            >,
        > {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = PackageGlobalSourcesElementTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct PackageRoundsRoundElementTypeSerializer<'ser> {
        pub(super) value: &'ser super::PackageRoundsRoundElementType,
        pub(super) state: ::std::boxed::Box<PackageRoundsRoundElementTypeSerializerState<'ser>>,
        pub(super) name: &'ser ::core::primitive::str,
        pub(super) is_root: ::core::primitive::bool,
    }
    #[derive(Debug)]
    pub(super) enum PackageRoundsRoundElementTypeSerializerState<'ser> {
        Init__,
        Info(
            ::xsd_parser_types::quick_xml::IterSerializer<
                'ser,
                ::core::option::Option<&'ser super::InfoType>,
                super::InfoType,
            >,
        ),
        Themes(
            ::xsd_parser_types::quick_xml::IterSerializer<
                'ser,
                ::core::option::Option<&'ser super::PackageRoundsRoundThemesElementType>,
                super::PackageRoundsRoundThemesElementType,
            >,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> PackageRoundsRoundElementTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::result::Result<
            ::core::option::Option<::xsd_parser_types::quick_xml::Event<'ser>>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            loop {
                match &mut *self.state {
                    PackageRoundsRoundElementTypeSerializerState::Init__ => {
                        *self.state = PackageRoundsRoundElementTypeSerializerState::Info(
                            ::xsd_parser_types::quick_xml::IterSerializer::new(
                                self.value.info.as_ref(),
                                Some("info"),
                                false,
                            ),
                        );
                        let mut bytes = ::xsd_parser_types::quick_xml::BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        helper.write_xmlns(&mut bytes, None, &super::NS_UNNAMED_2);
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&::xsd_parser_types::misc::NamespacePrefix::XSI),
                                &::xsd_parser_types::misc::Namespace::XSI,
                            );
                        }
                        helper.write_attrib(&mut bytes, "name", &self.value.name)?;
                        helper.write_attrib_opt(&mut bytes, "type", &self.value.type_)?;
                        return Ok(Some(::xsd_parser_types::quick_xml::Event::Start(bytes)));
                    }
                    PackageRoundsRoundElementTypeSerializerState::Info(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state = PackageRoundsRoundElementTypeSerializerState::Themes(
                                    ::xsd_parser_types::quick_xml::IterSerializer::new(
                                        self.value.themes.as_ref(),
                                        Some("themes"),
                                        false,
                                    ),
                                )
                            }
                        }
                    }
                    PackageRoundsRoundElementTypeSerializerState::Themes(x) => match x
                        .next(helper)
                        .transpose()?
                    {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state = PackageRoundsRoundElementTypeSerializerState::End__,
                    },
                    PackageRoundsRoundElementTypeSerializerState::End__ => {
                        *self.state = PackageRoundsRoundElementTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(::xsd_parser_types::quick_xml::Event::End(
                            ::xsd_parser_types::quick_xml::BytesEnd::new(self.name),
                        )));
                    }
                    PackageRoundsRoundElementTypeSerializerState::Done__ => return Ok(None),
                    PackageRoundsRoundElementTypeSerializerState::Phantom__(_) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> ::xsd_parser_types::quick_xml::Serializer<'ser>
        for PackageRoundsRoundElementTypeSerializer<'ser>
    {
        fn next(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::option::Option<
            ::core::result::Result<
                ::xsd_parser_types::quick_xml::Event<'ser>,
                ::xsd_parser_types::quick_xml::Error,
            >,
        > {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = PackageRoundsRoundElementTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct PackageRoundsRoundThemesElementTypeSerializer<'ser> {
        pub(super) value: &'ser super::PackageRoundsRoundThemesElementType,
        pub(super) state:
            ::std::boxed::Box<PackageRoundsRoundThemesElementTypeSerializerState<'ser>>,
        pub(super) name: &'ser ::core::primitive::str,
        pub(super) is_root: ::core::primitive::bool,
    }
    #[derive(Debug)]
    pub(super) enum PackageRoundsRoundThemesElementTypeSerializerState<'ser> {
        Init__,
        Theme(
            ::xsd_parser_types::quick_xml::IterSerializer<
                'ser,
                &'ser [super::PackageRoundsRoundThemesThemeElementType],
                super::PackageRoundsRoundThemesThemeElementType,
            >,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> PackageRoundsRoundThemesElementTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::result::Result<
            ::core::option::Option<::xsd_parser_types::quick_xml::Event<'ser>>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            loop {
                match &mut *self.state {
                    PackageRoundsRoundThemesElementTypeSerializerState::Init__ => {
                        *self.state = PackageRoundsRoundThemesElementTypeSerializerState::Theme(
                            ::xsd_parser_types::quick_xml::IterSerializer::new(
                                &self.value.theme[..],
                                Some("theme"),
                                false,
                            ),
                        );
                        let mut bytes = ::xsd_parser_types::quick_xml::BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        helper.write_xmlns(&mut bytes, None, &super::NS_UNNAMED_2);
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&::xsd_parser_types::misc::NamespacePrefix::XSI),
                                &::xsd_parser_types::misc::Namespace::XSI,
                            );
                        }
                        return Ok(Some(::xsd_parser_types::quick_xml::Event::Start(bytes)));
                    }
                    PackageRoundsRoundThemesElementTypeSerializerState::Theme(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state =
                                    PackageRoundsRoundThemesElementTypeSerializerState::End__
                            }
                        }
                    }
                    PackageRoundsRoundThemesElementTypeSerializerState::End__ => {
                        *self.state = PackageRoundsRoundThemesElementTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(::xsd_parser_types::quick_xml::Event::End(
                            ::xsd_parser_types::quick_xml::BytesEnd::new(self.name),
                        )));
                    }
                    PackageRoundsRoundThemesElementTypeSerializerState::Done__ => return Ok(None),
                    PackageRoundsRoundThemesElementTypeSerializerState::Phantom__(_) => {
                        unreachable!()
                    }
                }
            }
        }
    }
    impl<'ser> ::xsd_parser_types::quick_xml::Serializer<'ser>
        for PackageRoundsRoundThemesElementTypeSerializer<'ser>
    {
        fn next(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::option::Option<
            ::core::result::Result<
                ::xsd_parser_types::quick_xml::Event<'ser>,
                ::xsd_parser_types::quick_xml::Error,
            >,
        > {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = PackageRoundsRoundThemesElementTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct PackageRoundsRoundThemesThemeElementTypeSerializer<'ser> {
        pub(super) value: &'ser super::PackageRoundsRoundThemesThemeElementType,
        pub(super) state:
            ::std::boxed::Box<PackageRoundsRoundThemesThemeElementTypeSerializerState<'ser>>,
        pub(super) name: &'ser ::core::primitive::str,
        pub(super) is_root: ::core::primitive::bool,
    }
    #[derive(Debug)]
    pub(super) enum PackageRoundsRoundThemesThemeElementTypeSerializerState<'ser> {
        Init__,
        Info(
            ::xsd_parser_types::quick_xml::IterSerializer<
                'ser,
                ::core::option::Option<&'ser super::InfoType>,
                super::InfoType,
            >,
        ),
        Questions(
            ::xsd_parser_types::quick_xml::IterSerializer<
                'ser,
                ::core::option::Option<
                    &'ser super::PackageRoundsRoundThemesThemeQuestionsElementType,
                >,
                super::PackageRoundsRoundThemesThemeQuestionsElementType,
            >,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> PackageRoundsRoundThemesThemeElementTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::result::Result<
            ::core::option::Option<::xsd_parser_types::quick_xml::Event<'ser>>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            loop {
                match &mut *self.state {
                    PackageRoundsRoundThemesThemeElementTypeSerializerState::Init__ => {
                        *self.state = PackageRoundsRoundThemesThemeElementTypeSerializerState::Info(
                            ::xsd_parser_types::quick_xml::IterSerializer::new(
                                self.value.info.as_ref(),
                                Some("info"),
                                false,
                            ),
                        );
                        let mut bytes = ::xsd_parser_types::quick_xml::BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        helper.write_xmlns(&mut bytes, None, &super::NS_UNNAMED_2);
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&::xsd_parser_types::misc::NamespacePrefix::XSI),
                                &::xsd_parser_types::misc::Namespace::XSI,
                            );
                        }
                        helper.write_attrib(&mut bytes, "name", &self.value.name)?;
                        return Ok(Some(::xsd_parser_types::quick_xml::Event::Start(bytes)));
                    }
                    PackageRoundsRoundThemesThemeElementTypeSerializerState::Info(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => *self.state =
                                PackageRoundsRoundThemesThemeElementTypeSerializerState::Questions(
                                    ::xsd_parser_types::quick_xml::IterSerializer::new(
                                        self.value.questions.as_ref(),
                                        Some("questions"),
                                        false,
                                    ),
                                ),
                        }
                    }
                    PackageRoundsRoundThemesThemeElementTypeSerializerState::Questions(x) => {
                        match x.next(helper).transpose()? {
                            Some(event) => return Ok(Some(event)),
                            None => {
                                *self.state =
                                    PackageRoundsRoundThemesThemeElementTypeSerializerState::End__
                            }
                        }
                    }
                    PackageRoundsRoundThemesThemeElementTypeSerializerState::End__ => {
                        *self.state =
                            PackageRoundsRoundThemesThemeElementTypeSerializerState::Done__;
                        helper.end_ns_scope();
                        return Ok(Some(::xsd_parser_types::quick_xml::Event::End(
                            ::xsd_parser_types::quick_xml::BytesEnd::new(self.name),
                        )));
                    }
                    PackageRoundsRoundThemesThemeElementTypeSerializerState::Done__ => {
                        return Ok(None)
                    }
                    PackageRoundsRoundThemesThemeElementTypeSerializerState::Phantom__(_) => {
                        unreachable!()
                    }
                }
            }
        }
    }
    impl<'ser> ::xsd_parser_types::quick_xml::Serializer<'ser>
        for PackageRoundsRoundThemesThemeElementTypeSerializer<'ser>
    {
        fn next(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::option::Option<
            ::core::result::Result<
                ::xsd_parser_types::quick_xml::Event<'ser>,
                ::xsd_parser_types::quick_xml::Error,
            >,
        > {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state = PackageRoundsRoundThemesThemeElementTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct PackageRoundsRoundThemesThemeQuestionsElementTypeSerializer<'ser> {
        pub(super) value: &'ser super::PackageRoundsRoundThemesThemeQuestionsElementType,
        pub(super) state: ::std::boxed::Box<
            PackageRoundsRoundThemesThemeQuestionsElementTypeSerializerState<'ser>,
        >,
        pub(super) name: &'ser ::core::primitive::str,
        pub(super) is_root: ::core::primitive::bool,
    }
    #[derive(Debug)]
    pub(super) enum PackageRoundsRoundThemesThemeQuestionsElementTypeSerializerState<'ser> {
        Init__,
        Question(
            ::xsd_parser_types::quick_xml::IterSerializer<
                'ser,
                &'ser [super::PackageRoundsRoundThemesThemeQuestionsQuestionElementType],
                super::PackageRoundsRoundThemesThemeQuestionsQuestionElementType,
            >,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> PackageRoundsRoundThemesThemeQuestionsElementTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::result::Result<
            ::core::option::Option<::xsd_parser_types::quick_xml::Event<'ser>>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            loop {
                match &mut *self.state {
                    PackageRoundsRoundThemesThemeQuestionsElementTypeSerializerState::Init__ => {
                        * self . state = PackageRoundsRoundThemesThemeQuestionsElementTypeSerializerState :: Question (:: xsd_parser_types :: quick_xml :: IterSerializer :: new (& self . value . question [..] , Some ("question") , false)) ;
                        let mut bytes = ::xsd_parser_types::quick_xml::BytesStart::new(self.name);
                        helper.begin_ns_scope();
                        helper.write_xmlns(&mut bytes, None, &super::NS_UNNAMED_2);
                        if self.is_root {
                            helper.write_xmlns(
                                &mut bytes,
                                Some(&::xsd_parser_types::misc::NamespacePrefix::XSI),
                                &::xsd_parser_types::misc::Namespace::XSI,
                            );
                        }
                        return Ok(Some(::xsd_parser_types::quick_xml::Event::Start(bytes)));
                    }
                    PackageRoundsRoundThemesThemeQuestionsElementTypeSerializerState::Question(
                        x,
                    ) => match x.next(helper).transpose()? {
                        Some(event) => return Ok(Some(event)),
                        None => *self.state =
                            PackageRoundsRoundThemesThemeQuestionsElementTypeSerializerState::End__,
                    },
                    PackageRoundsRoundThemesThemeQuestionsElementTypeSerializerState::End__ => {
                        * self . state = PackageRoundsRoundThemesThemeQuestionsElementTypeSerializerState :: Done__ ;
                        helper.end_ns_scope();
                        return Ok(Some(::xsd_parser_types::quick_xml::Event::End(
                            ::xsd_parser_types::quick_xml::BytesEnd::new(self.name),
                        )));
                    }
                    PackageRoundsRoundThemesThemeQuestionsElementTypeSerializerState::Done__ => {
                        return Ok(None)
                    }
                    PackageRoundsRoundThemesThemeQuestionsElementTypeSerializerState::Phantom__(
                        _,
                    ) => unreachable!(),
                }
            }
        }
    }
    impl<'ser> ::xsd_parser_types::quick_xml::Serializer<'ser>
        for PackageRoundsRoundThemesThemeQuestionsElementTypeSerializer<'ser>
    {
        fn next(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::option::Option<
            ::core::result::Result<
                ::xsd_parser_types::quick_xml::Event<'ser>,
                ::xsd_parser_types::quick_xml::Error,
            >,
        > {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    *self.state =
                        PackageRoundsRoundThemesThemeQuestionsElementTypeSerializerState::Done__;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeSerializer<'ser> {
        pub(super) value: &'ser super::PackageRoundsRoundThemesThemeQuestionsQuestionElementType,
        pub(super) state: ::std::boxed::Box<
            PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeSerializerState<'ser>,
        >,
        pub(super) name: &'ser ::core::primitive::str,
        pub(super) is_root: ::core::primitive::bool,
    }
    #[derive(Debug)]
    pub(super) enum PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeSerializerState<'ser> {
        Init__ , Info (:: xsd_parser_types :: quick_xml :: IterSerializer < 'ser , :: core :: option :: Option < & 'ser super :: InfoType > , super :: InfoType >) , TypeName (:: xsd_parser_types :: quick_xml :: IterSerializer < 'ser , :: core :: option :: Option < & 'ser super :: PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameElementType > , super :: PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameElementType >) , Scenario (:: xsd_parser_types :: quick_xml :: IterSerializer < 'ser , :: core :: option :: Option < & 'ser super :: PackageRoundsRoundThemesThemeQuestionsQuestionScenarioElementType > , super :: PackageRoundsRoundThemesThemeQuestionsQuestionScenarioElementType >) , Script (:: xsd_parser_types :: quick_xml :: IterSerializer < 'ser , :: core :: option :: Option < & 'ser super :: PackageRoundsRoundThemesThemeQuestionsQuestionScriptElementType > , super :: PackageRoundsRoundThemesThemeQuestionsQuestionScriptElementType >) , Params (:: xsd_parser_types :: quick_xml :: IterSerializer < 'ser , :: core :: option :: Option < & 'ser super :: ParametersType > , super :: ParametersType >) , Right (< super :: PackageRoundsRoundThemesThemeQuestionsQuestionRightElementType as :: xsd_parser_types :: quick_xml :: WithSerializer > :: Serializer < 'ser >) , Wrong (:: xsd_parser_types :: quick_xml :: IterSerializer < 'ser , :: core :: option :: Option < & 'ser super :: PackageRoundsRoundThemesThemeQuestionsQuestionRightElementType > , super :: PackageRoundsRoundThemesThemeQuestionsQuestionRightElementType >) , End__ , Done__ , Phantom__ (& 'ser ()) , }
    impl<'ser> PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::result::Result<
            ::core::option::Option<::xsd_parser_types::quick_xml::Event<'ser>>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            loop {
                match & mut * self . state { PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeSerializerState :: Init__ => { * self . state = PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeSerializerState :: Info (:: xsd_parser_types :: quick_xml :: IterSerializer :: new (self . value . info . as_ref () , Some ("info") , false)) ; let mut bytes = :: xsd_parser_types :: quick_xml :: BytesStart :: new (self . name) ; helper . begin_ns_scope () ; helper . write_xmlns (& mut bytes , None , & super :: NS_UNNAMED_2) ; if self . is_root { helper . write_xmlns (& mut bytes , Some (& :: xsd_parser_types :: misc :: NamespacePrefix :: XSI) , & :: xsd_parser_types :: misc :: Namespace :: XSI) ; } helper . write_attrib (& mut bytes , "price" , & self . value . price) ? ; helper . write_attrib_opt (& mut bytes , "type" , & self . value . type_) ? ; return Ok (Some (:: xsd_parser_types :: quick_xml :: Event :: Start (bytes))) } PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeSerializerState :: Info (x) => match x . next (helper) . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeSerializerState :: TypeName (:: xsd_parser_types :: quick_xml :: IterSerializer :: new (self . value . type_name . as_ref () , Some ("type_name") , false)) , } PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeSerializerState :: TypeName (x) => match x . next (helper) . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeSerializerState :: Scenario (:: xsd_parser_types :: quick_xml :: IterSerializer :: new (self . value . scenario . as_ref () , Some ("scenario") , false)) , } PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeSerializerState :: Scenario (x) => match x . next (helper) . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeSerializerState :: Script (:: xsd_parser_types :: quick_xml :: IterSerializer :: new (self . value . script . as_ref () , Some ("script") , false)) , } PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeSerializerState :: Script (x) => match x . next (helper) . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeSerializerState :: Params (:: xsd_parser_types :: quick_xml :: IterSerializer :: new (self . value . params . as_ref () , Some ("params") , false)) , } PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeSerializerState :: Params (x) => match x . next (helper) . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeSerializerState :: Right (:: xsd_parser_types :: quick_xml :: WithSerializer :: serializer (& self . value . right , Some ("right") , false) ?) , } PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeSerializerState :: Right (x) => match x . next (helper) . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeSerializerState :: Wrong (:: xsd_parser_types :: quick_xml :: IterSerializer :: new (self . value . wrong . as_ref () , Some ("wrong") , false)) , } PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeSerializerState :: Wrong (x) => match x . next (helper) . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeSerializerState :: End__ , } PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeSerializerState :: End__ => { * self . state = PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeSerializerState :: Done__ ; helper . end_ns_scope () ; return Ok (Some (:: xsd_parser_types :: quick_xml :: Event :: End (:: xsd_parser_types :: quick_xml :: BytesEnd :: new (self . name)))) ; } PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeSerializerState :: Done__ => return Ok (None) , PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeSerializerState :: Phantom__ (_) => unreachable ! () , }
            }
        }
    }
    impl<'ser> ::xsd_parser_types::quick_xml::Serializer<'ser>
        for PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeSerializer<'ser>
    {
        fn next(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::option::Option<
            ::core::result::Result<
                ::xsd_parser_types::quick_xml::Event<'ser>,
                ::xsd_parser_types::quick_xml::Error,
            >,
        > {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    * self . state = PackageRoundsRoundThemesThemeQuestionsQuestionElementTypeSerializerState :: Done__ ;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameElementTypeSerializer<'ser> {
        pub(super) value:
            &'ser super::PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameElementType,
        pub(super) state: ::std::boxed::Box<
            PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameElementTypeSerializerState<'ser>,
        >,
        pub(super) name: &'ser ::core::primitive::str,
        pub(super) is_root: ::core::primitive::bool,
    }
    #[derive(Debug)]
    pub(super) enum PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameElementTypeSerializerState<
        'ser,
    > {
        Init__ , Param (:: xsd_parser_types :: quick_xml :: IterSerializer < 'ser , & 'ser [super :: PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameParamElementType] , super :: PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameParamElementType >) , End__ , Done__ , Phantom__ (& 'ser ()) , }
    impl<'ser> PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameElementTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::result::Result<
            ::core::option::Option<::xsd_parser_types::quick_xml::Event<'ser>>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            loop {
                match & mut * self . state { PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameElementTypeSerializerState :: Init__ => { * self . state = PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameElementTypeSerializerState :: Param (:: xsd_parser_types :: quick_xml :: IterSerializer :: new (& self . value . param [..] , Some ("param") , false)) ; let mut bytes = :: xsd_parser_types :: quick_xml :: BytesStart :: new (self . name) ; helper . begin_ns_scope () ; helper . write_xmlns (& mut bytes , None , & super :: NS_UNNAMED_2) ; if self . is_root { helper . write_xmlns (& mut bytes , Some (& :: xsd_parser_types :: misc :: NamespacePrefix :: XSI) , & :: xsd_parser_types :: misc :: Namespace :: XSI) ; } helper . write_attrib (& mut bytes , "name" , & self . value . name) ? ; return Ok (Some (:: xsd_parser_types :: quick_xml :: Event :: Start (bytes))) } PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameElementTypeSerializerState :: Param (x) => match x . next (helper) . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameElementTypeSerializerState :: End__ , } PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameElementTypeSerializerState :: End__ => { * self . state = PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameElementTypeSerializerState :: Done__ ; helper . end_ns_scope () ; return Ok (Some (:: xsd_parser_types :: quick_xml :: Event :: End (:: xsd_parser_types :: quick_xml :: BytesEnd :: new (self . name)))) ; } PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameElementTypeSerializerState :: Done__ => return Ok (None) , PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameElementTypeSerializerState :: Phantom__ (_) => unreachable ! () , }
            }
        }
    }
    impl<'ser> ::xsd_parser_types::quick_xml::Serializer<'ser>
        for PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameElementTypeSerializer<'ser>
    {
        fn next(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::option::Option<
            ::core::result::Result<
                ::xsd_parser_types::quick_xml::Event<'ser>,
                ::xsd_parser_types::quick_xml::Error,
            >,
        > {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    * self . state = PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameElementTypeSerializerState :: Done__ ;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct PackageRoundsRoundThemesThemeQuestionsQuestionScenarioElementTypeSerializer<'ser> {
        pub(super) value:
            &'ser super::PackageRoundsRoundThemesThemeQuestionsQuestionScenarioElementType,
        pub(super) state: ::std::boxed::Box<
            PackageRoundsRoundThemesThemeQuestionsQuestionScenarioElementTypeSerializerState<'ser>,
        >,
        pub(super) name: &'ser ::core::primitive::str,
        pub(super) is_root: ::core::primitive::bool,
    }
    #[derive(Debug)]
    pub(super) enum PackageRoundsRoundThemesThemeQuestionsQuestionScenarioElementTypeSerializerState<
        'ser,
    > {
        Init__ , Atom (:: xsd_parser_types :: quick_xml :: IterSerializer < 'ser , & 'ser [super :: PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomElementType] , super :: PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomElementType >) , End__ , Done__ , Phantom__ (& 'ser ()) , }
    impl<'ser> PackageRoundsRoundThemesThemeQuestionsQuestionScenarioElementTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::result::Result<
            ::core::option::Option<::xsd_parser_types::quick_xml::Event<'ser>>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            loop {
                match & mut * self . state { PackageRoundsRoundThemesThemeQuestionsQuestionScenarioElementTypeSerializerState :: Init__ => { * self . state = PackageRoundsRoundThemesThemeQuestionsQuestionScenarioElementTypeSerializerState :: Atom (:: xsd_parser_types :: quick_xml :: IterSerializer :: new (& self . value . atom [..] , Some ("atom") , false)) ; let mut bytes = :: xsd_parser_types :: quick_xml :: BytesStart :: new (self . name) ; helper . begin_ns_scope () ; helper . write_xmlns (& mut bytes , None , & super :: NS_UNNAMED_2) ; if self . is_root { helper . write_xmlns (& mut bytes , Some (& :: xsd_parser_types :: misc :: NamespacePrefix :: XSI) , & :: xsd_parser_types :: misc :: Namespace :: XSI) ; } return Ok (Some (:: xsd_parser_types :: quick_xml :: Event :: Start (bytes))) } PackageRoundsRoundThemesThemeQuestionsQuestionScenarioElementTypeSerializerState :: Atom (x) => match x . next (helper) . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = PackageRoundsRoundThemesThemeQuestionsQuestionScenarioElementTypeSerializerState :: End__ , } PackageRoundsRoundThemesThemeQuestionsQuestionScenarioElementTypeSerializerState :: End__ => { * self . state = PackageRoundsRoundThemesThemeQuestionsQuestionScenarioElementTypeSerializerState :: Done__ ; helper . end_ns_scope () ; return Ok (Some (:: xsd_parser_types :: quick_xml :: Event :: End (:: xsd_parser_types :: quick_xml :: BytesEnd :: new (self . name)))) ; } PackageRoundsRoundThemesThemeQuestionsQuestionScenarioElementTypeSerializerState :: Done__ => return Ok (None) , PackageRoundsRoundThemesThemeQuestionsQuestionScenarioElementTypeSerializerState :: Phantom__ (_) => unreachable ! () , }
            }
        }
    }
    impl<'ser> ::xsd_parser_types::quick_xml::Serializer<'ser>
        for PackageRoundsRoundThemesThemeQuestionsQuestionScenarioElementTypeSerializer<'ser>
    {
        fn next(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::option::Option<
            ::core::result::Result<
                ::xsd_parser_types::quick_xml::Event<'ser>,
                ::xsd_parser_types::quick_xml::Error,
            >,
        > {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    * self . state = PackageRoundsRoundThemesThemeQuestionsQuestionScenarioElementTypeSerializerState :: Done__ ;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct PackageRoundsRoundThemesThemeQuestionsQuestionScriptElementTypeSerializer<'ser> {
        pub(super) value:
            &'ser super::PackageRoundsRoundThemesThemeQuestionsQuestionScriptElementType,
        pub(super) state: ::std::boxed::Box<
            PackageRoundsRoundThemesThemeQuestionsQuestionScriptElementTypeSerializerState<'ser>,
        >,
        pub(super) name: &'ser ::core::primitive::str,
        pub(super) is_root: ::core::primitive::bool,
    }
    #[derive(Debug)]
    pub(super) enum PackageRoundsRoundThemesThemeQuestionsQuestionScriptElementTypeSerializerState<
        'ser,
    > {
        Init__,
        Step(
            ::xsd_parser_types::quick_xml::IterSerializer<
                'ser,
                &'ser [super::ParametersType],
                super::ParametersType,
            >,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> PackageRoundsRoundThemesThemeQuestionsQuestionScriptElementTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::result::Result<
            ::core::option::Option<::xsd_parser_types::quick_xml::Event<'ser>>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            loop {
                match & mut * self . state { PackageRoundsRoundThemesThemeQuestionsQuestionScriptElementTypeSerializerState :: Init__ => { * self . state = PackageRoundsRoundThemesThemeQuestionsQuestionScriptElementTypeSerializerState :: Step (:: xsd_parser_types :: quick_xml :: IterSerializer :: new (& self . value . step [..] , Some ("step") , false)) ; let mut bytes = :: xsd_parser_types :: quick_xml :: BytesStart :: new (self . name) ; helper . begin_ns_scope () ; helper . write_xmlns (& mut bytes , None , & super :: NS_UNNAMED_2) ; if self . is_root { helper . write_xmlns (& mut bytes , Some (& :: xsd_parser_types :: misc :: NamespacePrefix :: XSI) , & :: xsd_parser_types :: misc :: Namespace :: XSI) ; } return Ok (Some (:: xsd_parser_types :: quick_xml :: Event :: Start (bytes))) } PackageRoundsRoundThemesThemeQuestionsQuestionScriptElementTypeSerializerState :: Step (x) => match x . next (helper) . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = PackageRoundsRoundThemesThemeQuestionsQuestionScriptElementTypeSerializerState :: End__ , } PackageRoundsRoundThemesThemeQuestionsQuestionScriptElementTypeSerializerState :: End__ => { * self . state = PackageRoundsRoundThemesThemeQuestionsQuestionScriptElementTypeSerializerState :: Done__ ; helper . end_ns_scope () ; return Ok (Some (:: xsd_parser_types :: quick_xml :: Event :: End (:: xsd_parser_types :: quick_xml :: BytesEnd :: new (self . name)))) ; } PackageRoundsRoundThemesThemeQuestionsQuestionScriptElementTypeSerializerState :: Done__ => return Ok (None) , PackageRoundsRoundThemesThemeQuestionsQuestionScriptElementTypeSerializerState :: Phantom__ (_) => unreachable ! () , }
            }
        }
    }
    impl<'ser> ::xsd_parser_types::quick_xml::Serializer<'ser>
        for PackageRoundsRoundThemesThemeQuestionsQuestionScriptElementTypeSerializer<'ser>
    {
        fn next(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::option::Option<
            ::core::result::Result<
                ::xsd_parser_types::quick_xml::Event<'ser>,
                ::xsd_parser_types::quick_xml::Error,
            >,
        > {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    * self . state = PackageRoundsRoundThemesThemeQuestionsQuestionScriptElementTypeSerializerState :: Done__ ;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct PackageRoundsRoundThemesThemeQuestionsQuestionRightElementTypeSerializer<'ser> {
        pub(super) value:
            &'ser super::PackageRoundsRoundThemesThemeQuestionsQuestionRightElementType,
        pub(super) state: ::std::boxed::Box<
            PackageRoundsRoundThemesThemeQuestionsQuestionRightElementTypeSerializerState<'ser>,
        >,
        pub(super) name: &'ser ::core::primitive::str,
        pub(super) is_root: ::core::primitive::bool,
    }
    #[derive(Debug)]
    pub(super) enum PackageRoundsRoundThemesThemeQuestionsQuestionRightElementTypeSerializerState<
        'ser,
    > {
        Init__,
        Answer(
            ::xsd_parser_types::quick_xml::IterSerializer<
                'ser,
                &'ser [::std::string::String],
                ::std::string::String,
            >,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> PackageRoundsRoundThemesThemeQuestionsQuestionRightElementTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::result::Result<
            ::core::option::Option<::xsd_parser_types::quick_xml::Event<'ser>>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            loop {
                match & mut * self . state { PackageRoundsRoundThemesThemeQuestionsQuestionRightElementTypeSerializerState :: Init__ => { * self . state = PackageRoundsRoundThemesThemeQuestionsQuestionRightElementTypeSerializerState :: Answer (:: xsd_parser_types :: quick_xml :: IterSerializer :: new (& self . value . answer [..] , Some ("answer") , false)) ; let mut bytes = :: xsd_parser_types :: quick_xml :: BytesStart :: new (self . name) ; helper . begin_ns_scope () ; helper . write_xmlns (& mut bytes , None , & super :: NS_UNNAMED_2) ; if self . is_root { helper . write_xmlns (& mut bytes , Some (& :: xsd_parser_types :: misc :: NamespacePrefix :: XSI) , & :: xsd_parser_types :: misc :: Namespace :: XSI) ; } return Ok (Some (:: xsd_parser_types :: quick_xml :: Event :: Start (bytes))) } PackageRoundsRoundThemesThemeQuestionsQuestionRightElementTypeSerializerState :: Answer (x) => match x . next (helper) . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = PackageRoundsRoundThemesThemeQuestionsQuestionRightElementTypeSerializerState :: End__ , } PackageRoundsRoundThemesThemeQuestionsQuestionRightElementTypeSerializerState :: End__ => { * self . state = PackageRoundsRoundThemesThemeQuestionsQuestionRightElementTypeSerializerState :: Done__ ; helper . end_ns_scope () ; return Ok (Some (:: xsd_parser_types :: quick_xml :: Event :: End (:: xsd_parser_types :: quick_xml :: BytesEnd :: new (self . name)))) ; } PackageRoundsRoundThemesThemeQuestionsQuestionRightElementTypeSerializerState :: Done__ => return Ok (None) , PackageRoundsRoundThemesThemeQuestionsQuestionRightElementTypeSerializerState :: Phantom__ (_) => unreachable ! () , }
            }
        }
    }
    impl<'ser> ::xsd_parser_types::quick_xml::Serializer<'ser>
        for PackageRoundsRoundThemesThemeQuestionsQuestionRightElementTypeSerializer<'ser>
    {
        fn next(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::option::Option<
            ::core::result::Result<
                ::xsd_parser_types::quick_xml::Event<'ser>,
                ::xsd_parser_types::quick_xml::Error,
            >,
        > {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    * self . state = PackageRoundsRoundThemesThemeQuestionsQuestionRightElementTypeSerializerState :: Done__ ;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameParamElementTypeSerializer<
        'ser,
    > {
        pub(super) value:
            &'ser super::PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameParamElementType,
        pub(super) state: ::std::boxed::Box<
            PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameParamElementTypeSerializerState<
                'ser,
            >,
        >,
        pub(super) name: &'ser ::core::primitive::str,
        pub(super) is_root: ::core::primitive::bool,
    }
    #[derive(Debug)]
    pub(super) enum PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameParamElementTypeSerializerState<
        'ser,
    > {
        Init__,
        Content__(
            <::std::string::String as ::xsd_parser_types::quick_xml::WithSerializer>::Serializer<
                'ser,
            >,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameParamElementTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::result::Result<
            ::core::option::Option<::xsd_parser_types::quick_xml::Event<'ser>>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            loop {
                match & mut * self . state { PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameParamElementTypeSerializerState :: Init__ => { * self . state = PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameParamElementTypeSerializerState :: Content__ (:: xsd_parser_types :: quick_xml :: WithSerializer :: serializer (& self . value . content , None , false) ?) ; let mut bytes = :: xsd_parser_types :: quick_xml :: BytesStart :: new (self . name) ; helper . begin_ns_scope () ; helper . write_xmlns (& mut bytes , None , & super :: NS_UNNAMED_2) ; if self . is_root { helper . write_xmlns (& mut bytes , Some (& :: xsd_parser_types :: misc :: NamespacePrefix :: XSI) , & :: xsd_parser_types :: misc :: Namespace :: XSI) ; } helper . write_attrib (& mut bytes , "name" , & self . value . name) ? ; return Ok (Some (:: xsd_parser_types :: quick_xml :: Event :: Start (bytes))) } PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameParamElementTypeSerializerState :: Content__ (x) => match x . next (helper) . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameParamElementTypeSerializerState :: End__ , } PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameParamElementTypeSerializerState :: End__ => { * self . state = PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameParamElementTypeSerializerState :: Done__ ; helper . end_ns_scope () ; return Ok (Some (:: xsd_parser_types :: quick_xml :: Event :: End (:: xsd_parser_types :: quick_xml :: BytesEnd :: new (self . name)))) ; } PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameParamElementTypeSerializerState :: Done__ => return Ok (None) , PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameParamElementTypeSerializerState :: Phantom__ (_) => unreachable ! () , }
            }
        }
    }
    impl<'ser> ::xsd_parser_types::quick_xml::Serializer<'ser>
        for PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameParamElementTypeSerializer<'ser>
    {
        fn next(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::option::Option<
            ::core::result::Result<
                ::xsd_parser_types::quick_xml::Event<'ser>,
                ::xsd_parser_types::quick_xml::Error,
            >,
        > {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    * self . state = PackageRoundsRoundThemesThemeQuestionsQuestionTypeNameParamElementTypeSerializerState :: Done__ ;
                    Some(Err(error))
                }
            }
        }
    }
    #[derive(Debug)]
    pub struct PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomElementTypeSerializer<'ser> {
        pub(super) value:
            &'ser super::PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomElementType,
        pub(super) state: ::std::boxed::Box<
            PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomElementTypeSerializerState<
                'ser,
            >,
        >,
        pub(super) name: &'ser ::core::primitive::str,
        pub(super) is_root: ::core::primitive::bool,
    }
    #[derive(Debug)]
    pub(super) enum PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomElementTypeSerializerState<
        'ser,
    > {
        Init__,
        Content__(
            <::std::string::String as ::xsd_parser_types::quick_xml::WithSerializer>::Serializer<
                'ser,
            >,
        ),
        End__,
        Done__,
        Phantom__(&'ser ()),
    }
    impl<'ser> PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomElementTypeSerializer<'ser> {
        fn next_event(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::result::Result<
            ::core::option::Option<::xsd_parser_types::quick_xml::Event<'ser>>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            loop {
                match & mut * self . state { PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomElementTypeSerializerState :: Init__ => { * self . state = PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomElementTypeSerializerState :: Content__ (:: xsd_parser_types :: quick_xml :: WithSerializer :: serializer (& self . value . content , None , false) ?) ; let mut bytes = :: xsd_parser_types :: quick_xml :: BytesStart :: new (self . name) ; helper . begin_ns_scope () ; helper . write_xmlns (& mut bytes , None , & super :: NS_UNNAMED_2) ; if self . is_root { helper . write_xmlns (& mut bytes , Some (& :: xsd_parser_types :: misc :: NamespacePrefix :: XSI) , & :: xsd_parser_types :: misc :: Namespace :: XSI) ; } helper . write_attrib_opt (& mut bytes , "type" , & self . value . type_) ? ; helper . write_attrib_opt (& mut bytes , "time" , & self . value . time) ? ; return Ok (Some (:: xsd_parser_types :: quick_xml :: Event :: Start (bytes))) } PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomElementTypeSerializerState :: Content__ (x) => match x . next (helper) . transpose () ? { Some (event) => return Ok (Some (event)) , None => * self . state = PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomElementTypeSerializerState :: End__ , } PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomElementTypeSerializerState :: End__ => { * self . state = PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomElementTypeSerializerState :: Done__ ; helper . end_ns_scope () ; return Ok (Some (:: xsd_parser_types :: quick_xml :: Event :: End (:: xsd_parser_types :: quick_xml :: BytesEnd :: new (self . name)))) ; } PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomElementTypeSerializerState :: Done__ => return Ok (None) , PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomElementTypeSerializerState :: Phantom__ (_) => unreachable ! () , }
            }
        }
    }
    impl<'ser> ::xsd_parser_types::quick_xml::Serializer<'ser>
        for PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomElementTypeSerializer<'ser>
    {
        fn next(
            &mut self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::option::Option<
            ::core::result::Result<
                ::xsd_parser_types::quick_xml::Event<'ser>,
                ::xsd_parser_types::quick_xml::Error,
            >,
        > {
            match self.next_event(helper) {
                Ok(Some(event)) => Some(Ok(event)),
                Ok(None) => None,
                Err(error) => {
                    * self . state = PackageRoundsRoundThemesThemeQuestionsQuestionScenarioAtomElementTypeSerializerState :: Done__ ;
                    Some(Err(error))
                }
            }
        }
    }
}
pub mod xs {
    #[derive(Debug, Default)]
    pub struct EntitiesType(pub ::std::vec::Vec<::std::string::String>);
    impl ::xsd_parser_types::quick_xml::SerializeBytes for EntitiesType {
        fn serialize_bytes(
            &self,
            helper: &mut ::xsd_parser_types::quick_xml::SerializeHelper,
        ) -> ::core::result::Result<
            ::core::option::Option<::std::borrow::Cow<'_, ::core::primitive::str>>,
            ::xsd_parser_types::quick_xml::Error,
        > {
            if self.0.is_empty() {
                return Ok(None);
            }
            let mut data = ::std::string::String::new();
            for item in &self.0 {
                if let Some(bytes) = item.serialize_bytes(helper)? {
                    if !data.is_empty() {
                        data.push(' ');
                    }
                    data.push_str(&bytes);
                }
            }
            Ok(Some(::std::borrow::Cow::Owned(data)))
        }
    }
    impl ::xsd_parser_types::quick_xml::DeserializeBytes for EntitiesType {
        fn deserialize_bytes(
            helper: &mut ::xsd_parser_types::quick_xml::DeserializeHelper,
            bytes: &[::core::primitive::u8],
        ) -> ::core::result::Result<Self, ::xsd_parser_types::quick_xml::Error> {
            Ok(Self(
                bytes
                    .split(|b| *b == b' ' || *b == b'|' || *b == b',' || *b == b';')
                    .map(|bytes| ::std::string::String::deserialize_bytes(helper, bytes))
                    .collect::<::core::result::Result<::std::vec::Vec<_>, _>>()?,
            ))
        }
    }
    pub type EntityType = EntitiesType;
    pub type IdType = ::std::string::String;
    pub type IdrefType = ::std::string::String;
    pub type IdrefsType = EntitiesType;
    pub type NcNameType = ::std::string::String;
    pub type NmtokenType = ::std::string::String;
    pub type NmtokensType = EntitiesType;
    pub type NotationType = ::std::string::String;
    pub type NameType = ::std::string::String;
    pub type QNameType = ::std::string::String;
    pub type AnySimpleType = ::std::string::String;
    pub type AnyUriType = ::std::string::String;
    pub type Base64BinaryType = ::std::string::String;
    pub type BooleanType = ::core::primitive::bool;
    pub type ByteType = ::core::primitive::i8;
    pub type DateType = ::std::string::String;
    pub type DateTimeType = ::std::string::String;
    pub type DecimalType = ::core::primitive::f64;
    pub type DoubleType = ::core::primitive::f64;
    pub type DurationType = ::std::string::String;
    pub type FloatType = ::core::primitive::f32;
    pub type GDayType = ::std::string::String;
    pub type GMonthType = ::std::string::String;
    pub type GMonthDayType = ::std::string::String;
    pub type GYearType = ::std::string::String;
    pub type GYearMonthType = ::std::string::String;
    pub type HexBinaryType = ::std::string::String;
    pub type IntType = ::core::primitive::i32;
    pub type IntegerType = ::core::primitive::i32;
    pub type LanguageType = ::std::string::String;
    pub type LongType = ::core::primitive::i64;
    pub type NegativeIntegerType = ::core::primitive::isize;
    pub type NonNegativeIntegerType = ::core::primitive::usize;
    pub type NonPositiveIntegerType = ::core::primitive::isize;
    pub type NormalizedStringType = ::std::string::String;
    pub type PositiveIntegerType = ::core::primitive::usize;
    pub type ShortType = ::core::primitive::i16;
    pub type StringType = ::std::string::String;
    pub type TimeType = ::std::string::String;
    pub type TokenType = ::std::string::String;
    pub type UnsignedByteType = ::core::primitive::u8;
    pub type UnsignedIntType = ::core::primitive::u32;
    pub type UnsignedLongType = ::core::primitive::u64;
    pub type UnsignedShortType = ::core::primitive::u16;
}
