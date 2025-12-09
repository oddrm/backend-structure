pub mod queries;

// getentries(searchstring, sortby, ascending, page, pagesize) -> Vec<{entryid, name, path, platform, size, tags: vec<string>}>
// getsequences(entryID) -> Map<(sequenceID, sequence)>
// batchfetchsequences(vec<entryID>) -> Map<entryID, Map<(sequenceID, sequence)>>
// getmetadata(entryID) -> Metadata
// updatemetadata(entryID, Metadata) -> void
// updatetags(entryID, vec<string>) -> void
// addsequence(entryID, sequence) -> sequenceID
// removesequence(sequenceID) -> void
// updatesequence(sequenceID, sequence) -> void
