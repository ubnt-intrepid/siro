//! Managed effects.

/// The base trait that represents managed effects.
pub trait Effects {
    type Ok;
    type Error;

    fn end(self) -> Result<Self::Ok, Self::Error>;
}

/// A mix-in that controls DOM focuses.
pub trait DomFocus: Effects {
    /// Find DOM node by id string and focus on it.
    #[allow(unused_variables)]
    fn focus(&mut self, target_id: &str) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Find DOM node by id string and make it lose focus.
    #[allow(unused_variables)]
    fn blur(&mut self, target_id: &str) -> Result<(), Self::Error> {
        Ok(())
    }
}

/// A mix-in that controls navigations.
pub trait Navigation: Effects {
    #[allow(unused_variables)]
    fn push_url(&mut self, url: &str) -> Result<(), Self::Error> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn replace_url(&mut self, url: &str) -> Result<(), Self::Error> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn back(&mut self, count: usize) -> Result<(), Self::Error> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn forward(&mut self, count: usize) -> Result<(), Self::Error> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn load(&mut self, url: &str) -> Result<(), Self::Error> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn reload(&mut self, skip_cache: bool) -> Result<(), Self::Error> {
        Ok(())
    }
}
