pub mod errors;

pub trait JavaString {
    fn value_of<T>(x: T) -> Self
    where
        T: ToString;
    fn transform<R>(&self, function: fn(String) -> R) -> R;
    fn indent(&self, amount: usize) -> Self;
    fn is_blank(&self) -> bool;
    fn split_with_delimiters<T>(&self, regex: T) -> Vec<&str>
    where
        T: FnMut(char) -> bool;
    fn replace_all<T>(&self, regex: T, replacement: &Self) -> Self
    where
        T: FnMut(char) -> bool;
    fn replace_first<T: FnMut(char) -> bool>(&self, regex: T, replacement: &Self) -> Self;
    fn concat(&self, other: &Self) -> Self;
    fn subsequence(&self, begin_index: usize, end_index: usize) -> Vec<char>;
    fn substring(&self, begin_index: usize, end_index: usize) -> String;
    fn last_index_from<T: FnMut(char) -> bool>(&self, ch: T, from_index: usize) -> Option<usize>;
    fn last_index_of<T: FnMut(char) -> bool>(&self, ch: T) -> Option<usize>;
    fn index_of_between<T: FnMut(char) -> bool>(
        &self,
        ch: T,
        begin_index: usize,
        end_index: usize,
    ) -> Option<usize>;
    fn index_of<T: FnMut(char) -> bool>(&self, ch: T) -> Option<usize>;
    fn hash_code(&self) -> u32;
    fn region_matches(
        &self,
        toffset: usize,
        other: &Self,
        ooffset: usize,
        len: usize,
        ignore_case: bool,
    ) -> bool;
    fn compare_to_ignore_case(&self, other: &Self) -> i8;
    const COMPACT_STRINGS: bool;

    fn length(&self) -> usize;
    fn char_at(&self, index: usize) -> Option<char>;

    fn code_point_at(&self, index: usize) -> u8 {
        self.char_at(index).unwrap_or('\0') as u8
    }

    // TODO: code_point_before
    fn code_point_count(&self, begin_index: usize, end_index: usize) -> Option<usize>;
    // TODO: offset_by_code_points
    // TODO: get_chars
    fn get_chars(&self, src_begin: usize, src_end: usize, dest: &mut Vec<char>, dest_begin: usize);

    fn equals(&self, other: &Self) -> bool
    where
        Self: Eq,
    {
        self == other
    }

    fn equals_ignore_case(&self, other: &Self) -> bool;
    fn compare_to(&self, other: &Self) -> i8;
}

impl JavaString for String {
    const COMPACT_STRINGS: bool = true;

    fn length(&self) -> usize {
        self.len()
    }

    fn char_at(&self, index: usize) -> Option<char> {
        self.chars().nth(index)
    }

    fn code_point_count(&self, begin_index: usize, end_index: usize) -> Option<usize> {
        if begin_index.max(end_index) > self.length() {
            None
        } else {
            Some(end_index - begin_index)
        }
    }

    fn get_chars(
        &self,
        _src_begin: usize,
        _src_end: usize,
        _dest: &mut Vec<char>,
        _dest_begin: usize,
    ) {
        todo!();
        //dest[dest_begin..] = self.chars().collect::<Vec<char>>()[src_begin..src_end];
    }

    fn equals_ignore_case(&self, other: &Self) -> bool {
        self.to_lowercase() == other.to_lowercase()
    }

    fn compare_to(&self, other: &Self) -> i8 {
        match self.cmp(other) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        }
    }

    fn compare_to_ignore_case(&self, other: &Self) -> i8 {
        match self.to_lowercase().cmp(&other.to_lowercase()) {
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        }
    }

    fn region_matches(
        &self,
        toffset: usize,
        other: &Self,
        ooffset: usize,
        len: usize,
        ignore_case: bool,
    ) -> bool {
        if ignore_case {
            self.to_lowercase()[toffset..(toffset + len)]
                == other[ooffset..(ooffset + len)].to_lowercase()
        } else {
            self[toffset..(toffset + len)] == other[ooffset..(ooffset + len)]
        }
    }

    fn hash_code(&self) -> u32 {
        todo!()
    }

    fn index_of<T: FnMut(char) -> bool>(&self, ch: T) -> Option<usize> {
        self.find(ch)
    }

    fn index_of_between<T: FnMut(char) -> bool>(
        &self,
        ch: T,
        begin_index: usize,
        end_index: usize,
    ) -> Option<usize> {
        self[begin_index..end_index].find(ch)
    }

    fn last_index_of<T: FnMut(char) -> bool>(&self, ch: T) -> Option<usize> {
        self.rfind(ch)
    }

    fn last_index_from<T: FnMut(char) -> bool>(&self, ch: T, from_index: usize) -> Option<usize> {
        self[from_index..].rfind(ch)
    }

    fn substring(&self, begin_index: usize, end_index: usize) -> String {
        self[begin_index..end_index].to_string()
    }

    fn subsequence(&self, begin_index: usize, end_index: usize) -> Vec<char> {
        self[begin_index..end_index].chars().collect()
    }

    fn concat(&self, other: &Self) -> Self {
        let mut return_value = self.clone();
        return_value.push_str(other);
        return_value
    }

    fn replace_first<T: FnMut(char) -> bool>(&self, regex: T, replacement: &Self) -> Self {
        self.replacen(regex, replacement, 1)
    }

    fn replace_all<T: FnMut(char) -> bool>(&self, regex: T, replacement: &Self) -> Self {
        self.replace(regex, replacement)
    }

    fn split_with_delimiters<T: FnMut(char) -> bool>(&self, regex: T) -> Vec<&str> {
        self.split_inclusive(regex).collect()
    }

    fn is_blank(&self) -> bool {
        self.trim().is_empty()
    }

    fn indent(&self, amount: usize) -> Self {
        let mut return_value = String::new();

        for line in self.lines() {
            return_value.push_str(&" ".repeat(amount));
            return_value.push_str(line);
            return_value.push('\n');
        }

        return_value
    }

    // TODO: strip_indent(&self) -> Self
    // TODO: translate_escapes(&self) -> Self

    fn transform<R>(&self, function: fn(String) -> R) -> R {
        function(self.clone())
    }

    fn value_of<T: ToString>(x: T) -> Self {
        x.to_string()
    }
}
