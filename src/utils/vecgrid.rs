#[derive(Debug)]
pub struct VecGrid<T> {
    data: Vec<Option<T>>,
    width: usize,
    height: usize,
}

#[allow(dead_code)]
impl<T> VecGrid<T> {
    pub fn new(width: usize, height: usize) -> VecGrid<T> {
        let mut data = Vec::with_capacity(width * height);

        for _i in 0..width * height {
            data.push(None);
        }

        VecGrid { data, width, height }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.data
            .get(x + (y * self.width))
            .and_then(std::option::Option::as_ref)
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.data
            .get_mut(x + (y * self.width))
            .and_then(std::option::Option::as_mut)
    }

    pub fn set(&mut self, cell: T, x: usize, y: usize) {
        self.data[x + (y * self.width)] = Some(cell);
    }

    pub fn delete(&mut self, x: usize, y: usize) {
        self.data[x + (y * self.width)] = None;
    }

    pub fn get_data(&self) -> &Vec<Option<T>> {
        &self.data
    }

    pub fn set_data(&mut self, data: Vec<Option<T>>, width: usize, height: usize) {
        self.data = data;
        self.width = width;
        self.height = height;
    }
}
