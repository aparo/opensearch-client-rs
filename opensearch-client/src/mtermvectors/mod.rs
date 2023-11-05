use crate::OsClient;
mod builder;
mod types;
pub struct Mtermvectors<'a> {
  os_client: &'a OsClient,
}

impl<'a> Mtermvectors<'a> {
  pub fn new(os_client: &'a OsClient) -> Self {
    Self { os_client }
  }

  ///Returns multiple termvectors in one request.
  ///
  ///Sends a `GET` request to `/_mtermvectors`
  ///
  ///Arguments:
  /// - `field_statistics`: Specifies if document count, sum of document
  ///   frequencies and sum of total term frequencies should be returned.
  ///   Applies to all returned documents unless otherwise specified in body
  ///   'params' or 'docs'.
  /// - `fields`: Comma-separated list of fields to return. Applies to all
  ///   returned documents unless otherwise specified in body 'params' or
  ///   'docs'.
  /// - `ids`: Comma-separated list of documents ids. You must define ids as
  ///   parameter or set 'ids' or 'docs' in the request body.
  /// - `offsets`: Specifies if term offsets should be returned. Applies to all
  ///   returned documents unless otherwise specified in body 'params' or
  ///   'docs'.
  /// - `payloads`: Specifies if term payloads should be returned. Applies to
  ///   all returned documents unless otherwise specified in body 'params' or
  ///   'docs'.
  /// - `positions`: Specifies if term positions should be returned. Applies to
  ///   all returned documents unless otherwise specified in body 'params' or
  ///   'docs'.
  /// - `preference`: Specify the node or shard the operation should be
  ///   performed on. Applies to all returned documents unless otherwise
  ///   specified in body 'params' or 'docs'.
  /// - `realtime`: Specifies if requests are real-time as opposed to
  ///   near-real-time.
  /// - `routing`: Routing value. Applies to all returned documents unless
  ///   otherwise specified in body 'params' or 'docs'.
  /// - `term_statistics`: Specifies if total term frequency and document
  ///   frequency should be returned. Applies to all returned documents unless
  ///   otherwise specified in body 'params' or 'docs'.
  /// - `version`: Explicit version number for concurrency control.
  /// - `version_type`: Specific version type.
  ///```ignore
  /// let response = client.mtermvectors().get()
  ///    .field_statistics(field_statistics)
  ///    .fields(fields)
  ///    .ids(ids)
  ///    .offsets(offsets)
  ///    .payloads(payloads)
  ///    .positions(positions)
  ///    .preference(preference)
  ///    .realtime(realtime)
  ///    .routing(routing)
  ///    .term_statistics(term_statistics)
  ///    .version(version)
  ///    .version_type(version_type)
  ///    .send()
  ///    .await;
  /// ```
  pub fn get(&self) -> builder::MtermvectorsGet {
    builder::MtermvectorsGet::new(self.os_client)
  }

  ///Returns multiple termvectors in one request.
  ///
  ///Sends a `POST` request to `/_mtermvectors`
  ///
  ///Arguments:
  /// - `field_statistics`: Specifies if document count, sum of document
  ///   frequencies and sum of total term frequencies should be returned.
  ///   Applies to all returned documents unless otherwise specified in body
  ///   'params' or 'docs'.
  /// - `fields`: Comma-separated list of fields to return. Applies to all
  ///   returned documents unless otherwise specified in body 'params' or
  ///   'docs'.
  /// - `ids`: Comma-separated list of documents ids. You must define ids as
  ///   parameter or set 'ids' or 'docs' in the request body.
  /// - `offsets`: Specifies if term offsets should be returned. Applies to all
  ///   returned documents unless otherwise specified in body 'params' or
  ///   'docs'.
  /// - `payloads`: Specifies if term payloads should be returned. Applies to
  ///   all returned documents unless otherwise specified in body 'params' or
  ///   'docs'.
  /// - `positions`: Specifies if term positions should be returned. Applies to
  ///   all returned documents unless otherwise specified in body 'params' or
  ///   'docs'.
  /// - `preference`: Specify the node or shard the operation should be
  ///   performed on. Applies to all returned documents unless otherwise
  ///   specified in body 'params' or 'docs'.
  /// - `realtime`: Specifies if requests are real-time as opposed to
  ///   near-real-time.
  /// - `routing`: Routing value. Applies to all returned documents unless
  ///   otherwise specified in body 'params' or 'docs'.
  /// - `term_statistics`: Specifies if total term frequency and document
  ///   frequency should be returned. Applies to all returned documents unless
  ///   otherwise specified in body 'params' or 'docs'.
  /// - `version`: Explicit version number for concurrency control.
  /// - `version_type`: Specific version type.
  /// - `body`
  ///```ignore
  /// let response = client.mtermvectors().post()
  ///    .field_statistics(field_statistics)
  ///    .fields(fields)
  ///    .ids(ids)
  ///    .offsets(offsets)
  ///    .payloads(payloads)
  ///    .positions(positions)
  ///    .preference(preference)
  ///    .realtime(realtime)
  ///    .routing(routing)
  ///    .term_statistics(term_statistics)
  ///    .version(version)
  ///    .version_type(version_type)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn post(&self) -> builder::MtermvectorsPost {
    builder::MtermvectorsPost::new(self.os_client)
  }

  ///Returns multiple termvectors in one request.
  ///
  ///Sends a `GET` request to `/{index}/_mtermvectors`
  ///
  ///Arguments:
  /// - `index`: The index in which the document resides.
  /// - `field_statistics`: Specifies if document count, sum of document
  ///   frequencies and sum of total term frequencies should be returned.
  ///   Applies to all returned documents unless otherwise specified in body
  ///   'params' or 'docs'.
  /// - `fields`: Comma-separated list of fields to return. Applies to all
  ///   returned documents unless otherwise specified in body 'params' or
  ///   'docs'.
  /// - `ids`: Comma-separated list of documents ids. You must define ids as
  ///   parameter or set 'ids' or 'docs' in the request body.
  /// - `offsets`: Specifies if term offsets should be returned. Applies to all
  ///   returned documents unless otherwise specified in body 'params' or
  ///   'docs'.
  /// - `payloads`: Specifies if term payloads should be returned. Applies to
  ///   all returned documents unless otherwise specified in body 'params' or
  ///   'docs'.
  /// - `positions`: Specifies if term positions should be returned. Applies to
  ///   all returned documents unless otherwise specified in body 'params' or
  ///   'docs'.
  /// - `preference`: Specify the node or shard the operation should be
  ///   performed on. Applies to all returned documents unless otherwise
  ///   specified in body 'params' or 'docs'.
  /// - `realtime`: Specifies if requests are real-time as opposed to
  ///   near-real-time.
  /// - `routing`: Routing value. Applies to all returned documents unless
  ///   otherwise specified in body 'params' or 'docs'.
  /// - `term_statistics`: Specifies if total term frequency and document
  ///   frequency should be returned. Applies to all returned documents unless
  ///   otherwise specified in body 'params' or 'docs'.
  /// - `version`: Explicit version number for concurrency control.
  /// - `version_type`: Specific version type.
  ///```ignore
  /// let response = client.mtermvectors().get_with_index()
  ///    .index(index)
  ///    .field_statistics(field_statistics)
  ///    .fields(fields)
  ///    .ids(ids)
  ///    .offsets(offsets)
  ///    .payloads(payloads)
  ///    .positions(positions)
  ///    .preference(preference)
  ///    .realtime(realtime)
  ///    .routing(routing)
  ///    .term_statistics(term_statistics)
  ///    .version(version)
  ///    .version_type(version_type)
  ///    .send()
  ///    .await;
  /// ```
  pub fn get_with_index(&self) -> builder::MtermvectorsGetWithIndex {
    builder::MtermvectorsGetWithIndex::new(self.os_client)
  }

  ///Returns multiple termvectors in one request.
  ///
  ///Sends a `POST` request to `/{index}/_mtermvectors`
  ///
  ///Arguments:
  /// - `index`: The index in which the document resides.
  /// - `field_statistics`: Specifies if document count, sum of document
  ///   frequencies and sum of total term frequencies should be returned.
  ///   Applies to all returned documents unless otherwise specified in body
  ///   'params' or 'docs'.
  /// - `fields`: Comma-separated list of fields to return. Applies to all
  ///   returned documents unless otherwise specified in body 'params' or
  ///   'docs'.
  /// - `ids`: Comma-separated list of documents ids. You must define ids as
  ///   parameter or set 'ids' or 'docs' in the request body.
  /// - `offsets`: Specifies if term offsets should be returned. Applies to all
  ///   returned documents unless otherwise specified in body 'params' or
  ///   'docs'.
  /// - `payloads`: Specifies if term payloads should be returned. Applies to
  ///   all returned documents unless otherwise specified in body 'params' or
  ///   'docs'.
  /// - `positions`: Specifies if term positions should be returned. Applies to
  ///   all returned documents unless otherwise specified in body 'params' or
  ///   'docs'.
  /// - `preference`: Specify the node or shard the operation should be
  ///   performed on. Applies to all returned documents unless otherwise
  ///   specified in body 'params' or 'docs'.
  /// - `realtime`: Specifies if requests are real-time as opposed to
  ///   near-real-time.
  /// - `routing`: Routing value. Applies to all returned documents unless
  ///   otherwise specified in body 'params' or 'docs'.
  /// - `term_statistics`: Specifies if total term frequency and document
  ///   frequency should be returned. Applies to all returned documents unless
  ///   otherwise specified in body 'params' or 'docs'.
  /// - `version`: Explicit version number for concurrency control.
  /// - `version_type`: Specific version type.
  /// - `body`
  ///```ignore
  /// let response = client.mtermvectors().post_with_index()
  ///    .index(index)
  ///    .field_statistics(field_statistics)
  ///    .fields(fields)
  ///    .ids(ids)
  ///    .offsets(offsets)
  ///    .payloads(payloads)
  ///    .positions(positions)
  ///    .preference(preference)
  ///    .realtime(realtime)
  ///    .routing(routing)
  ///    .term_statistics(term_statistics)
  ///    .version(version)
  ///    .version_type(version_type)
  ///    .body(body)
  ///    .send()
  ///    .await;
  /// ```
  pub fn post_with_index(&self) -> builder::MtermvectorsPostWithIndex {
    builder::MtermvectorsPostWithIndex::new(self.os_client)
  }
}
