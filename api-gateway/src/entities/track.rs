use std::convert::From;

use super::Document;
use crate::graphql::schema::Context;

#[derive(Debug, Clone)]
/// Track of a document
pub struct Track {
    // ID of the track
    pub id: i32,
    // Number of the track
    pub track_number: i32,
    // Title of the track
    pub title: String,
    // Path for the track's audio
    pub audio_path: String,
    // ID of the track's document
    pub document_id: i32,
}

#[derive(juniper::GraphQLInputObject, Debug, Clone)]
// Retrieving tracks for a document
pub struct DocumentTracks {
    // Document to query for tracks
    pub document_id: i32,
    // Limit for the query
    pub limit: i32,
    // Offset for the query
    pub offset: i32,
}

#[juniper::graphql_object(Context = Context)]
impl Track {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn track_number(&self) -> i32 {
        self.track_number
    }

    pub fn title(&self) -> &str {
        self.title.as_str()
    }

    pub fn audio_path(&self) -> &str {
        self.audio_path.as_str()
    }

    pub async fn document(&self, context: &Context) -> Document {
        context
            .document_data
            .as_ref()
            .unwrap()
            .documents_by_id(self.document_id)
            .await
    }
}

impl From<schema::courses::Track> for Track {
    fn from(x: schema::courses::Track) -> Self {
        Self {
            id: x.id,
            track_number: x.track_number,
            title: x.title,
            audio_path: x.audio_path,
            document_id: x.document_id,
        }
    }
}
