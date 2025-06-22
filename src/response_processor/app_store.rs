use quick_xml::events::attributes::Attribute;
use quick_xml::events::Event;
use quick_xml::name::QName;
use quick_xml::Reader;
use serde::{Deserialize, Serialize};

use crate::{
    errors::CrawlerError,
    response_processor::traits::{TExtractData, TStoreType},
    OUTPUT_PATH,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppStoreReview {
    pub date: String,
    pub star: i32,
    pub like: i32,
    pub dislike: i32,
    pub title: String,
    pub review: String,
}

impl AppStoreReview {
    pub fn new() -> Self {
        Self {
            date: String::new(),
            star: 0,
            like: 0,
            dislike: 0,
            title: String::new(),
            review: String::new(),
        }
    }
}

impl TStoreType for AppStoreReview {
    fn get_output_path(&self, app_id: &str) -> String {
        format!("{}/app_store/{}.csv", OUTPUT_PATH, app_id)
    }
}

impl TExtractData for AppStoreReview {
    fn extract_data(&self, response: &[u8]) -> Result<Vec<Self>, CrawlerError> {
        tracing::debug!("Starting XML parsing with quick-xml");

        let mut reader = Reader::from_reader(response);
        reader.trim_text(true);

        let mut buf: Vec<u8> = Vec::new();
        let mut reviews = Vec::new();

        // 임시로 필드를 담을 변수들
        let mut current = AppStoreReview::new();
        let mut in_entry = false;

        loop {
            match reader.read_event() {
                Ok(Event::Start(ref e)) => {
                    Self::handle_start_event(e, &mut reader, &mut current, &mut in_entry);
                }

                Ok(Event::End(ref e)) if e.name() == QName(b"entry") => {
                    // push the completed review
                    tracing::debug!("Exit </entry>: {:?}", current);
                    // if the required fields (title, review) are present, push
                    if !current.title.is_empty() && !current.review.is_empty() {
                        reviews.push(current.clone());
                    } else {
                        tracing::debug!("Skipped incomplete entry");
                    }
                    in_entry = false;
                }

                Ok(Event::Eof) => {
                    tracing::debug!("XML parsing completed. Found {} reviews", reviews.len());
                    break;
                }

                Err(e) => {
                    tracing::error!("XML parsing error: {}", e);
                    return Err(CrawlerError::Parse(e.to_string()));
                }

                _ => {}
            }
            buf.clear();
        }

        Ok(reviews)
    }
}

impl AppStoreReview {
    fn handle_start_event(
        e: &quick_xml::events::BytesStart,
        reader: &mut Reader<&[u8]>,
        current: &mut AppStoreReview,
        in_entry: &mut bool,
    ) {
        match e.name() {
            QName(b"entry") => {
                *in_entry = true;
                *current = AppStoreReview::new();
                tracing::debug!("Enter <entry>");
            }
            QName(b"title") if *in_entry => {
                Self::read_text_field(reader, e.name(), &mut current.title);
            }
            QName(b"content") if *in_entry => {
                Self::handle_content_element(reader, e, current);
            }
            QName(b"im:rating") if *in_entry => {
                Self::read_numeric_field(reader, e.name(), &mut current.star);
            }
            QName(b"im:voteSum") if *in_entry => {
                Self::read_numeric_field(reader, e.name(), &mut current.like);
            }
            QName(b"im:voteCount") if *in_entry => {
                Self::handle_vote_count(reader, e.name(), current);
            }
            QName(b"updated") if *in_entry => {
                Self::read_text_field(reader, e.name(), &mut current.date);
            }
            _ => {}
        }
    }

    fn read_text_field(reader: &mut Reader<&[u8]>, name: QName, field: &mut String) {
        if let Ok(txt) = reader.read_text(name) {
            *field = txt.to_string();
        }
    }

    fn read_numeric_field(reader: &mut Reader<&[u8]>, name: QName, field: &mut i32) {
        if let Ok(txt) = reader.read_text(name) {
            *field = txt.parse().unwrap_or(0);
        }
    }

    fn handle_content_element(
        reader: &mut Reader<&[u8]>,
        e: &quick_xml::events::BytesStart,
        current: &mut AppStoreReview,
    ) {
        // type="text"인 content만
        if let Some(Attribute { key: _, value: _ }) = e
            .attributes()
            .filter_map(Result::ok)
            .find(|attr| attr.key == QName(b"type") && attr.value.as_ref() == b"text")
        {
            Self::read_text_field(reader, e.name(), &mut current.review);
        }
    }

    fn handle_vote_count(reader: &mut Reader<&[u8]>, name: QName, current: &mut AppStoreReview) {
        if let Ok(txt) = reader.read_text(name) {
            let total: i32 = txt.parse().unwrap_or(0);
            current.dislike = total.saturating_sub(current.like);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_extract_app_store_reviews() {
        let xml_content = r#"
        <feed xmlns:im="http://itunes.apple.com/rss" xmlns="http://www.w3.org/2005/Atom" xml:lang="en">
            <id>https://itunes.apple.com/us/rss/customerreviews/id=1194408342/sortby=mostrecent/xml</id>
            <title>iTunes Store: Customer Reviews</title>
            <updated>2025-06-22T11:36:11-07:00</updated>
            <entry>
                <id>12645174720</id>
                <title>Great idea but not well executed.</title>
                <content type="text">If you are test, this isn't it. It could be—and I wish it was. But the clothing choices are not accurate, and the comfort level adjustments are way off. I tried to adjust my comfort level to be warmer by one level and it changed the clothing from a sleeveless short dress to long pants and a long sleeved flannel. Neither of those is appropriate for 71 degrees F. Disappointing to say the least.</content>
                <im:contentType term="Application" label="Application"/>
                <im:voteSum>0</im:voteSum>
                <im:voteCount>0</im:voteCount>
                <im:rating>2</im:rating>
                <updated>2025-05-11T10:19:38-07:00</updated>
                <im:version>7.2.3</im:version>
                <author>
                    <name>Beegirl200073?4!/9</name>
                    <uri>https://itunes.apple.com/us/reviews/id167338708</uri>
                </author>
            </entry>
            <entry>
                <id>12484337193</id>
                <title>Love it!!</title>
                <content type="text">Super helpful and cute! You do have to pay for the subscription to be able to use the widget feature, but I think it's worth the price for the functionality of it.</content>
                <im:contentType term="Application" label="Application"/>
                <im:voteSum>0</im:voteSum>
                <im:voteCount>0</im:voteCount>
                <im:rating>4</im:rating>
                <updated>2025-03-30T15:13:14-07:00</updated>
                <im:version>7.2.2</im:version>
                <author>
                    <name>LenaM720</name>
                    <uri>https://itunes.apple.com/us/reviews/id108277834</uri>
                </author>
            </entry>
        </feed>
        "#;

        let extractor = AppStoreReview::new();
        let result = extractor.extract_data(xml_content.as_bytes());

        assert!(result.is_ok());
        let reviews = result.unwrap();

        assert_eq!(reviews.len(), 2);

        let first = &reviews[0];
        assert_eq!(first.title, "Great idea but not well executed.");
        assert_eq!(first.star, 2);
        assert_eq!(first.like, 0);
        assert_eq!(first.dislike, 0);
        assert_eq!(first.date, "2025-05-11T10:19:38-07:00");
        assert!(first.review.contains("If you are test, this isn't it"));

        let second = &reviews[1];
        assert_eq!(second.title, "Love it!!");
        assert_eq!(second.star, 4);
        assert_eq!(second.like, 0);
        assert_eq!(second.dislike, 0);
        assert_eq!(second.date, "2025-03-30T15:13:14-07:00");
        assert!(second.review.contains("Super helpful and cute!"));
    }

    #[test]
    fn test_extract_data_with_empty_xml() {
        let xml_content = r#"
        <feed xmlns:im="http://itunes.apple.com/rss" xmlns="http://www.w3.org/2005/Atom" xml:lang="en">
            <id>https://itunes.apple.com/us/rss/customerreviews/id=1194408342/sortby=mostrecent/xml</id>
            <title>iTunes Store: Customer Reviews</title>
            <updated>2025-06-22T11:36:11-07:00</updated>
        </feed>
        "#;

        let extractor = AppStoreReview::new();
        let result = extractor.extract_data(xml_content.as_bytes()).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_extract_data_with_invalid_xml() {
        let invalid_bytes = vec![0xFF, 0xFE, 0x00, 0x00];
        let extractor = AppStoreReview::new();
        let result = extractor.extract_data(&invalid_bytes);
        // quick-xml handles invalid input gracefully and returns empty result
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }
}
