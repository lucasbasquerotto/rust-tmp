#[rocket::async_trait]
pub trait AsyncFrom<T> {
	async fn from(from: T) -> Self;
}

#[rocket::async_trait]
pub trait AsyncInto<T> {
	async fn into(self) -> T;
}

#[rocket::async_trait]
impl<A: Send, B: AsyncFrom<A>> AsyncInto<B> for A {
	async fn into(self) -> B {
		B::from(self).await
	}
}
