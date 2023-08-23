struct ObjectPool<T> {
    objects: Vec<T>
}

impl<T> ObjectPool<T> {
    fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    fn acquire(&mut self) -> Option<T> {
        self.objects.pop()
    }

    fn release(&mut self, object: T) {
        self.objects.push(object);
    }
}