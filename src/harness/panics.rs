use std::any::Any;
use std::panic;
use std::panic::UnwindSafe;
use std::sync::{Arc, Mutex};

/// Calls the provided function and wraps its result in `Some`. If it panics due
/// to a `todo!()` call, then returns `None`. Any other panic proceeds normally.
pub fn catching_todo<F, O>(f: F) -> Option<O>
where
    F: UnwindSafe + Fn() -> O,
{
    let old_hook = Arc::new(Mutex::new(Some(panic::take_hook())));
    panic::set_hook({
        let old_hook = old_hook.clone();
        Box::new(move |info| {
            if !is_todo(info.payload()) {
                if let Some(hook) = &*old_hook.lock().unwrap() {
                    hook(info)
                }
            }
        })
    });
    let result = panic::catch_unwind(f);
    let old_hook = old_hook.lock().unwrap().take().unwrap();
    panic::set_hook(old_hook);
    match result {
        Ok(o) => Some(o),
        Err(error) => {
            if is_todo(&*error) {
                None
            } else {
                panic::resume_unwind(error)
            }
        }
    }
}

fn is_todo(error: &dyn Any) -> bool {
    if let Some(&s) = error.downcast_ref::<&str>() {
        s == "not yet implemented"
    } else if let Some(s) = error.downcast_ref::<String>() {
        s.starts_with("not yet implemented: ")
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catching_todo_on_no_panic() {
        let result = catching_todo(|| 42);
        assert_eq!(result, Some(42));
    }

    #[test]
    fn catching_todo_catches_todo() {
        let result = catching_todo(|| todo!());
        assert_eq!(result, None);
    }

    #[test]
    fn catching_todo_catches_todo_with_message() {
        let result = catching_todo(|| todo!("stuff"));
        assert_eq!(result, None);
    }

    #[test]
    #[should_panic]
    fn catching_todo_allows_other_panics() {
        catching_todo(|| unimplemented!());
    }
}
