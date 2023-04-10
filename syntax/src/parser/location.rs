use std::ops::Range;

#[derive(Clone)]
pub struct SyntaxLocation {
    line: usize,
    range: Range<usize>,
}

impl SyntaxLocation {
    pub fn new(line: usize, range: Range<usize>) -> Self {
        Self { line, range }
    }

    /// Not zero-copy safe - it clones the `Range`.
    fn get_range_as_owned(&self) -> Range<usize> {
        self.range.clone()
    }

    /// Zero-copy safe
    pub fn get_range_as_ref(&self) -> &Range<usize> {
        &self.range
    }
}

pub struct LocationsDatabase {
    locations: Vec<SyntaxLocation>,
}

impl LocationsDatabase {
    pub fn new() -> Self {
        Self { locations: vec![] }
    }

    pub fn push(&mut self, location: SyntaxLocation) {
        self.locations.push(location)
    }

    pub fn push_raw(&mut self, line: usize, range: Range<usize>) {
        self.locations.push(SyntaxLocation { line, range })
    }

    #[inline]
    pub fn get_line_from_absolute_offset(&self, offset: &usize) -> Option<usize> {
        match self
            .locations
            .iter()
            .filter(|x| x.get_range_as_ref().contains(offset))
            .next()
        {
            Some(sl) => Some(sl.line),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LocationsDatabase;

    #[test]
    fn lines() {
        let mut loc_db = LocationsDatabase::new();
        loc_db.push(super::SyntaxLocation {
            line: 1,
            range: 0..16,
        });
        loc_db.push(super::SyntaxLocation {
            line: 2,
            range: 17..24,
        });
        loc_db.push(super::SyntaxLocation {
            line: 3,
            range: 25..32,
        });

        let mut line = loc_db.get_line_from_absolute_offset(&7).unwrap();
        assert_eq!(line, 1);

        line = loc_db.get_line_from_absolute_offset(&18).unwrap();
        assert_eq!(line, 2);

        line = loc_db.get_line_from_absolute_offset(&28).unwrap();
        assert_eq!(line, 3);
    }
}
