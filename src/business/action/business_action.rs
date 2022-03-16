impl Exception<Option<ErrorData>> for BusinessException<UserRequestContext> {
	fn handle(self) -> Option<ErrorData> {
		let _ = &self.error();
		self.public
	}
}

// use std::fmt::Debug;

// use crate::lib::{
// 	core::action::Exception,
// 	core::action::{ActionScope, ActionType, RequestContext, RequestInput},
// };

// use super::action_data::{BusinessException, ErrorData};

// pub trait BusinessActionType<C, I>: PartialEq + Eq + Debug
// where
// 	C: RequestContext,
// 	I: PartialEq + Eq + Debug,
// {
// 	fn scope() -> ActionScope;
// 	fn id(&self) -> I;
// 	fn validate(&self, info: &C) -> Result<(), BusinessException<C>>;
// }

// impl<C, T> ActionType<C, BusinessException<C>> for T
// where
// 	C: RequestContext,
// 	T: BusinessActionType<C, u32>,
// {
// 	fn scope() -> ActionScope {
// 		Self::scope()
// 	}

// 	fn validate(&self, info: &C) -> Result<(), BusinessException<C>> {
// 		self.validate(info)
// 	}
// }

// pub trait BusinessAction<
// 	C: RequestContext,
// 	I: Debug,
// 	O: Debug,
// 	E: Exception<Option<ErrorData>>,
// 	T: BusinessActionType<C, u32>,
// >
// {
// 	fn action_type() -> T;
// 	fn new(input: RequestInput<I, C>) -> Self;
// 	fn input(&self) -> &RequestInput<I, C>;
// 	fn run(self) -> Result<O, E>;
// }

// impl<I, D, O, T, A> Action<RequestInput<D, I>, O, Option<ErrorData>, BusinessException<I>> for A
// where
// 	I: Debug,
// 	O: Debug,
// 	T: ActionType<I, BusinessException<I>>,
// 	A: BusinessAction<I, D, O, BusinessException<I>, T>,
// {
// 	fn new(input: RequestInput<D, I>) -> Self {
// 		Self::new(input)
// 	}

// 	fn run(self) -> Result<O, BusinessException<I>> {
// 		Self::action_type().validate(&self.input().info)?;
// 		self.run()
// 	}
// }

// #[derive(Debug)]
// pub struct ModeratorRequestContext {
// 	pub application: Application,
// 	pub session: ModeratorSession,
// 	pub request: Request,
// }

// impl RequestContext for ModeratorRequestContext {}

// #[derive(Debug)]
// pub struct AutomaticRequestContext {
// 	pub application: Application,
// 	pub request: Request,
// }

// impl RequestContext for AutomaticRequestContext {}
