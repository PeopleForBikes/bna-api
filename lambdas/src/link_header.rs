/// This module is a loose implementation of the RFC 8288 describing web linking
/// (https://datatracker.ietf.org/doc/rfc8288/).
///
/// It is intended to be used to serialize or desserialize Link HTTP headers.
///
/// An example of such link can be found in the documentation of the GitHub REST API v3:
/// https://docs.github.com/en/rest/guides/using-pagination-in-the-rest-api?apiVersion=2022-11-28
use crate::nomstr;
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_while1},
    character::{
        complete::{space0, space1},
        is_alphabetic, is_space,
    },
    combinator::{map, map_res, value},
    error::Error,
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair},
    Finish, IResult,
};
use url::Url;

/// Represents a relation type.
///
/// The "rel" parameter is mandatory and must appear only once.
///
/// ## Example
/// ```
/// use lambdas::link_header::RelationType;
///
/// let rel = RelationType::try_from(r#";rel="prev""#).unwrap();
/// assert_eq!(rel.relation_type(), "prev");
/// ```
#[derive(Debug, PartialEq)]
pub struct RelationType<'a>(&'a str);

nomstr!(RelationType);

impl<'a> ToString for RelationType<'a> {
    fn to_string(&self) -> String {
        let rel_type = self.0;
        format!("rel=\"{rel_type}\"")
    }
}

impl<'a> RelationType<'a> {
    pub fn new(rel: &'a str) -> Self {
        RelationType(rel)
    }
    /// Recognizes a Relation type.
    ///
    /// ## Example
    /// ```
    /// use lambdas::link_header::RelationType;
    ///
    /// assert_eq!(RelationType::parse(r#" ; rel="prev""#), Ok(("", RelationType::new("prev"))));
    /// assert_eq!(RelationType::parse(r#" ; rel="alternate stylesheet""#), Ok(("", RelationType::new("alternate stylesheet"))));
    /// ```
    pub fn parse(i: &str) -> IResult<&str, RelationType> {
        let (i, _) = space0(i)?;
        let (i, _) = tag(";")(i)?;
        let (i, _) = space0(i)?;
        let (i, rel) = delimited(
            tag(r#"rel=""#),
            take_while1(|c| is_alphabetic(c as u8) || is_space(c as u8)),
            tag("\""),
        )(i)?;
        Ok((i, RelationType(rel)))
    }

    pub fn relation_type(&self) -> &'a str {
        self.0
    }
}

/// Represent a URI-Reference.
#[derive(Debug, PartialEq)]
pub struct LinkTarget(Url);

impl<'a> TryFrom<&'a str> for LinkTarget {
    type Error = Error<String>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match LinkTarget::parse(value).finish() {
            Ok((_, item)) => Ok(item),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

impl ToString for LinkTarget {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl LinkTarget {
    pub fn new(url: Url) -> Self {
        LinkTarget(url)
    }
    /// Recognizes and validate a URI.
    ///
    /// ```
    /// use lambdas::link_header::LinkTarget;
    /// use url::Url;
    ///
    /// assert_eq!(
    ///   LinkTarget::parse(r#"<https://api.github.com/repositories/1300192/issues?page=2>"#),
    ///   Ok(("", LinkTarget::new(Url::parse("https://api.github.com/repositories/1300192/issues?page=2").unwrap())))
    /// );
    /// ```
    pub fn parse(i: &str) -> IResult<&str, LinkTarget> {
        let (i, uri) = map_res(delimited(tag("<"), is_not(">"), tag(">")), move |uri| {
            Url::parse(uri)
        })(i)?;
        Ok((i, LinkTarget(uri)))
    }
}

/// Represent a Link.
///
/// A Link is a collection of link-values.
#[derive(Debug, PartialEq)]
pub struct Link<'a> {
    links: Vec<LinkValues<'a>>,
}

nomstr!(Link);

impl<'a> ToString for Link<'a> {
    fn to_string(&self) -> String {
        let lv: Vec<String> = self.links.iter().map(|l| l.to_string()).collect();
        let lv_str = lv.join(", ");
        format!("Link: {lv_str}")
    }
}

impl<'a> Link<'a> {
    /// Recognizes a Link.
    ///
    /// ```
    /// use lambdas::link_header::{Link, LinkValues, RelationType, LinkTarget};
    /// use url::Url;
    ///
    /// let link = Link::parse(r#"link: <https://api.github.com/repositories/1300192/issues?page=2>; rel="prev""#).unwrap();
    /// ```
    pub fn parse(i: &str) -> IResult<&str, Link> {
        let (i, _) = alt((tag("Link:"), tag("link:")))(i)?;
        let (i, _) = space1(i)?;
        let (i, links) = separated_list1(tag(", "), LinkValues::parse)(i)?;

        Ok((i, Link { links }))
    }

    /// Creates an empty Link.
    pub fn new() -> Self {
        Self { links: vec![] }
    }

    /// Adds a link-value entry from a str.
    pub fn add_link_value_from_str(&mut self, link_value: &'a str) -> Result<(), Error<String>> {
        let value = LinkValues::try_from(link_value)?;
        self.links.push(value);
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub struct Rel<'a>(&'a str);

#[derive(Debug, PartialEq)]
pub struct Anchor<'a>(&'a str);
#[derive(Debug, PartialEq)]
pub struct HRefLang<'a>(&'a str);
#[derive(Debug, PartialEq)]
pub struct Media<'a>(&'a str);
#[derive(Debug, PartialEq)]
pub struct Title<'a>(&'a str);
#[derive(Debug, PartialEq)]
pub struct TitleStar<'a>(&'a str);
#[derive(Debug, PartialEq)]
pub struct Type_<'a>(&'a str);

#[derive(Debug, PartialEq)]
pub struct LinkValues<'a> {
    uri: LinkTarget,
    rel: Rel<'a>,
    anchor: Option<Anchor<'a>>,
    href_lang: Option<HRefLang<'a>>,
    media: Option<Media<'a>>,
    title: Option<Title<'a>>,
    title_star: Option<TitleStar<'a>>,
    type_: Option<Type_<'a>>,
}

nomstr!(LinkValues);

impl<'a> ToString for LinkValues<'a> {
    fn to_string(&self) -> String {
        let uri_str = self.uri.to_string();
        let rel = format!(r#"; rel="{}""#, self.rel.0);
        let anchor = match &self.anchor {
            Some(s) => format!(r#"; rel="{}""#, s.0),
            None => String::from(""),
        };
        let href_lang = match &self.href_lang {
            Some(s) => format!(r#"; hreflang="{}""#, s.0),
            None => String::from(""),
        };
        let media = match &self.media {
            Some(s) => format!(r#"; media="{}""#, s.0),
            None => String::from(""),
        };
        let title = match &self.title {
            Some(s) => format!(r#"; title="{}""#, s.0),
            None => String::from(""),
        };
        let title_star = match &self.title_star {
            Some(s) => format!(r#"; title*="{}""#, s.0),
            None => String::from(""),
        };
        let type_ = match &self.type_ {
            Some(s) => format!(r#"; type_="{}""#, s.0),
            None => String::from(""),
        };
        format!("<{uri_str}>{rel}{anchor}{href_lang}{media}{title}{title_star}{type_}")
    }
}

impl<'a> LinkValues<'a> {
    pub fn new(uri: LinkTarget, rel: &'a str) -> Self {
        LinkValues {
            uri,
            rel: Rel(rel),
            anchor: None,
            href_lang: None,
            media: None,
            title: None,
            title_star: None,
            type_: None,
        }
    }

    pub fn try_from_link_params(link: LinkTarget, link_params: &[LinkParam<'a>]) -> Self {
        let mut iter = link_params.iter();
        let rel = iter
            .find(|p| p.target_attribute == TargetAttribute::Rel)
            .map(|p| Rel(p.token))
            .unwrap();
        LinkValues {
            uri: link,
            rel,
            anchor: iter
                .find(|p| p.target_attribute == TargetAttribute::Anchor)
                .map(|p| Anchor(p.token)),
            href_lang: iter
                .find(|p| p.target_attribute == TargetAttribute::HRefLang)
                .map(|p| HRefLang(p.token)),
            media: iter
                .find(|p| p.target_attribute == TargetAttribute::Media)
                .map(|p| Media(p.token)),
            title: iter
                .find(|p| p.target_attribute == TargetAttribute::Title)
                .map(|p| Title(p.token)),
            title_star: iter
                .find(|p| p.target_attribute == TargetAttribute::TitleStar)
                .map(|p| TitleStar(p.token)),
            type_: iter
                .find(|p| p.target_attribute == TargetAttribute::Type_)
                .map(|p| Type_(p.token)),
        }
    }

    pub fn set_value(&mut self, param: &'a LinkParam) {
        match param.target_attribute() {
            TargetAttribute::Anchor => self.anchor = Some(Anchor(param.token())),
            TargetAttribute::HRefLang => self.href_lang = Some(HRefLang(param.token())),
            TargetAttribute::Media => self.media = Some(Media(param.token())),
            TargetAttribute::Rel => (),
            TargetAttribute::Title => self.title = Some(Title(param.token())),
            TargetAttribute::TitleStar => self.title_star = Some(TitleStar(param.token())),
            TargetAttribute::Type_ => self.type_ = Some(Type_(param.token())),
        }
    }

    pub fn parse(i: &str) -> IResult<&str, LinkValues> {
        let (i, uri) = LinkTarget::parse(i)?;
        let (i, link_params) = many1(LinkParam::parse)(i)?;
        let lv = LinkValues::try_from_link_params(uri, &link_params);
        Ok((i, lv))
    }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TargetAttribute {
    Anchor,
    HRefLang,
    Media,
    Rel,
    Title,
    TitleStar,
    Type_,
}

fn target_attribute(i: &str) -> IResult<&str, TargetAttribute> {
    alt((
        value(TargetAttribute::Anchor, tag("anchor")),
        value(TargetAttribute::HRefLang, tag("hreflang")),
        value(TargetAttribute::Media, tag("media")),
        value(TargetAttribute::Rel, tag("rel")),
        value(TargetAttribute::Title, tag("title")),
        value(TargetAttribute::TitleStar, tag("title*")),
        value(TargetAttribute::Type_, tag("type")),
    ))(i)
}

fn token(i: &str) -> IResult<&str, &str> {
    delimited(tag("\""), is_not("\""), tag("\""))(i)
}

pub struct LinkParam<'a> {
    target_attribute: TargetAttribute,
    token: &'a str,
}

nomstr!(LinkParam);

impl<'a> LinkParam<'a> {
    pub fn target_attribute(&self) -> TargetAttribute {
        self.target_attribute
    }

    pub fn token(&self) -> &'a str {
        self.token
    }

    pub fn parse(i: &str) -> IResult<&str, LinkParam> {
        let (i, _) = space0(i)?;
        let (i, _) = tag(";")(i)?;
        let (i, _) = space0(i)?;
        map(
            separated_pair(target_attribute, tag("="), token),
            |(target_attribute, token)| LinkParam {
                target_attribute,
                token,
            },
        )(i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    const FULL_LINK: &'static str = r#"Link: <https://api.github.com/repositories/1300192/issues?page=2>; rel="prev", <https://api.github.com/repositories/1300192/issues?page=4>; rel="next", <https://api.github.com/repositories/1300192/issues?page=515>; rel="last", <https://api.github.com/repositories/1300192/issues?page=1>; rel="first""#;

    #[test]
    fn test_single_link_value() {
        let (_, actual) = Link::parse(
            r#"link: <https://api.github.com/repositories/1300192/issues?page=2>; rel="prev""#,
        )
        .unwrap();
        let links = vec![LinkValues::try_from(
            r#"<https://api.github.com/repositories/1300192/issues?page=2>; rel="prev""#,
        )
        .unwrap()];
        assert_eq!(actual, Link { links });
    }

    #[test]
    fn test_multiple_link_values() {
        let (_, actual) = Link::parse(FULL_LINK).unwrap();
        let mut expected = Link::new();
        expected
            .add_link_value_from_str(
                r#"<https://api.github.com/repositories/1300192/issues?page=2>; rel="prev""#,
            )
            .unwrap();
        expected
            .add_link_value_from_str(
                r#"<https://api.github.com/repositories/1300192/issues?page=4>; rel="next""#,
            )
            .unwrap();
        expected
            .add_link_value_from_str(
                r#"<https://api.github.com/repositories/1300192/issues?page=515>; rel="last""#,
            )
            .unwrap();
        expected
            .add_link_value_from_str(
                r#"<https://api.github.com/repositories/1300192/issues?page=1>; rel="first""#,
            )
            .unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn link_to_string() {
        let link = Link::try_from(FULL_LINK).unwrap();
        assert_eq!(link.to_string(), FULL_LINK);
    }

    #[rstest]
    #[case::rel(r#";rel="prev""#, TargetAttribute::Rel, "prev")]
    #[case::rel(
        r#"; rel="http://example.net/foo""#,
        TargetAttribute::Rel,
        "http://example.net/foo"
    )]
    #[case::title(
        r#" ; title="previous chapter"""#,
        TargetAttribute::Title,
        "previous chapter"
    )]
    #[case::anchor(";anchor=\"#foo\"", TargetAttribute::Anchor, "#foo")]
    fn test_parse_link_param(#[case] i: &str, #[case] ta: TargetAttribute, #[case] t: &str) {
        let actual = LinkParam::try_from(i).unwrap();
        assert_eq!(actual.target_attribute(), ta);
        assert_eq!(actual.token(), t);
    }

    #[rstest]
    #[case(
        r#"Link: <http://example.com/TheBook/chapter2>; rel="previous"; title="previous chapter""#
    )]
    #[case(r#"Link: <http://example.org/>; rel="start http://example.net/relation/other""#)]
    // #[case(r#"Link: </>; rel="http://example.net/foo""#)] // -> Cannot parse urls without a base
    // #[case(r##"Link: </terms>; rel="copyright"; anchor="#foo""##)] // -> Cannot parse urls without a base
    fn test_parsing_rfc_examples(#[case] i: &str) {
        Link::try_from(i).unwrap();
    }

    #[test]
    #[ignore = "Url::parse cannot parse relative url without a base."]
    fn test_link_value_00() {
        LinkTarget::parse("</terms>").unwrap();
    }
}
