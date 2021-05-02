pub struct DiskManager {
    // ヒープファイルのファイルディスクリプタ
    heap_file: File,
    // 裁判するページIDを決めるカウンタ
    next_page_id: u64,
}

impl DiskManager {
    // コンストラクタ
    pub fn new(data_file: File)-> io::Result<Self> {

    }
    // ファイルパスを指定して開く
    pub fn open(data_file_path: imple AsRef<Path>)-> io::Result<Self> {

    }
    // 新しいページIDを採番する
    pub fn allocate_page(&mut self)-> PageId {

    }
    // ページのデータを読み出す
    pub fn read_page_data(&mut self, page_id: PageId, data: &[u8])-> io::Result<()> {

    }
    // データをページに書き出す
    pub fn write_page_data(&mut self, page_id: PageId, data: &[u8])-> io::Result<()> {

    }
}