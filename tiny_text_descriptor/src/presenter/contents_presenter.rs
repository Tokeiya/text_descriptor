pub trait ContentsPresenter {
	fn expression(&self, trim: Option<usize>) -> String;
	fn len(&self) -> usize;
	fn is_empty(&self) -> bool;
}

