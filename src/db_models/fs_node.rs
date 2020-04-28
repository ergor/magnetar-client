
// TODO: why isnt u64 implemented for ToSql?
#[derive(Default, Debug)]
pub struct FsNode {
    pub id: i64,
    pub node_type: NodeType,
    pub name: String,
    pub size: i64,
    pub uid: u32,
    pub gid: u32,
    pub permissions: u32,
    pub creation_date: i64,
    pub modified_date: i64,
    pub path: String,
    pub sha1_checksum: String, // 40 chars
    pub links_to: String, // for soft links (symlinks)
    pub inode: i64,
    pub nlinks: i64, // number of hard links to this inode
    pub parent_id: i64, // fk: FsNode::id
}

#[derive(Debug)]
pub enum NodeType {
    File,
    Directory,
    Symlink,
    Other
}

impl NodeType {
    pub fn value(&self) -> i32 {
        match self {
            NodeType::File => 0,
            NodeType::Directory => 1,
            NodeType::Symlink => 2,
            NodeType::Other => 3,
        }
    }
}

impl Default for NodeType {
    fn default() -> Self {
        NodeType::File
    }
}

impl FsNode {

    pub fn new() -> FsNode {
        FsNode::default()
    }

    pub fn insert (&self, conn: &rusqlite::Connection) -> rusqlite::Result<()> {
        conn.execute(
            "INSERT INTO fs_node ( \
                    node_type, \
                    name, \
                    size, \
                    uid, \
                    gid, \
                    permissions, \
                    creation_date, \
                    modified_date, \
                    path, \
                    sha1_checksum, \
                    links_to, \
                    inode, \
                    nlinks, \
                    parent_id) \
                    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
            rusqlite::params![
                self.node_type.value(),
                self.name,
                self.size,
                self.uid,
                self.gid,
                self.permissions,
                self.creation_date,
                self.modified_date,
                self.path,
                self.sha1_checksum,
                self.links_to,
                self.inode,
                self.nlinks,
                self.parent_id
            ]
        )?;
        Ok(())
    }
}


