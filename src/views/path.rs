pub struct Path {
    pub prefix: String
}


impl Path {
    pub fn define(&self, following_path: String) -> String {
        //to_owned creates owned data from borrowed data by borrowing
        return self.prefix.to_owned() + &following_path
    }
}
