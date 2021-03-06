#[derive(Deserialize, Debug)]
/// Owner information for the object
pub struct Owner {
    #[serde(rename = "DisplayName")]
    /// Object owner's name.
    pub display_name: String,
    #[serde(rename = "ID")]
    /// Object owner's ID.
    pub id: String,
}

#[derive(Deserialize, Debug)]
/// An individual object in a ListBucketResult
pub struct Object {
    #[serde(rename = "LastModified")]
    /// Date and time the object was last modified.
    pub last_modified: String,
    #[serde(rename = "ETag")]
    /// The entity tag is an MD5 hash of the object. The ETag only reflects changes to the
    /// contents of an object, not its metadata.
    pub e_tag: String,
    #[serde(rename = "StorageClass")]
    /// STANDARD | STANDARD_IA | REDUCED_REDUNDANCY | GLACIER
    pub storage_class: String,
    #[serde(rename = "Key")]
    /// The object's key
    pub key: String,
    #[serde(rename = "Owner")]
    /// Bucket owner
    pub owner: Owner,
    #[serde(rename = "Size")]
    /// Size in bytes of the object.
    pub size: i32,
}

#[derive(Deserialize, Debug)]
/// The parsed result of a s3 bucket listing
pub struct ListBucketResult {
    #[serde(rename = "Name")]
    /// Name of the bucket.
    pub name: String,
    #[serde(rename = "NextMarker")]
    /// When the response is truncated (that is, the IsTruncated element value in the response
    /// is true), you can use the key name in this field as a marker in the subsequent request
    /// to get next set of objects. Amazon S3 lists objects in UTF-8 character encoding in
    /// lexicographical order.
    pub next_marker: String,
    #[serde(rename = "Delimiter")]
    /// A delimiter is a character you use to group keys.
    pub delimiter: String,
    #[serde(rename = "MaxKeys")]
    /// Sets the maximum number of keys returned in the response body.
    pub max_keys: i32,
    #[serde(rename = "Prefix")]
    /// Limits the response to keys that begin with the specified prefix.
    pub prefix: String,
    #[serde(rename = "Marker")]
    /// Indicates where in the bucket listing begins. Marker is included in the response if
    /// it was sent with the request.
    pub marker: String,
    #[serde(rename = "EncodingType")]
    /// Specifies the encoding method to used
    pub encoding_type: String,
    #[serde(rename = "IsTruncated")]
    ///  Specifies whether (true) or not (false) all of the results were returned.
    ///  If the number of results exceeds that specified by MaxKeys, all of the results
    ///  might not be returned.
    pub is_truncated: bool,
    #[serde(rename = "Contents")]
    /// Metadata about each object returned.
    pub contents: Vec<Object>,
    #[serde(rename = "CommonPrefixs")]
    /// All of the keys rolled up into a common prefix count as a single return when
    /// calculating the number of returns.
    pub common_prefixes: Vec<CommonPrefix>,
}

#[derive(Deserialize, Debug)]
/// CommonPrefix is used to group keys
pub struct CommonPrefix {
    #[serde(rename = "Prefix")]
    /// Keys that begin with the indicated prefix.
    pub prefix: String,
}
