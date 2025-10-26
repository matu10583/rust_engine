pub struct Events<T> {
    a: Vec<T>,
    b: Vec<T>,
    read_a: bool,
}

impl<T> Events<T> {
    pub fn new() -> Self {
        Self {
            a: Vec::new(),
            b: Vec::new(),
            read_a: true,
        }
    }

    pub fn send(&mut self, evt: T) {
        self.write_mut().push(evt);
    }

    pub fn extend<I: IntoIterator<Item = T>>(&mut self, it: I) {
        self.write_mut().extend(it);
    }

    pub fn is_empty(&self) -> bool {
        self.read_ref().is_empty()
    }

    pub fn len(&self) -> usize {
        self.read_ref().len()
    }

    pub fn drain(&mut self) -> impl Iterator<Item = T> + '_ {
        self.read_mut().drain(..)
    }

    pub fn update(&mut self) {
        self.read_a = !self.read_a;
        self.write_mut().clear();
    }

    fn read_ref(&self) -> &Vec<T> {
        if self.read_a {
            &self.a
        } else {
            &self.b
        }
    }
    fn read_mut(&mut self) -> &mut Vec<T> {
        if self.read_a {
            &mut self.a
        } else {
            &mut self.b
        }
    }
    fn write_mut(&mut self) -> &mut Vec<T> {
        if self.read_a {
            &mut self.b
        } else {
            &mut self.a
        }
    }
}
