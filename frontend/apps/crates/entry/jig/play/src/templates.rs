use simple_html_template::TemplateCache;

use std::fmt;

thread_local! {
    pub static TEMPLATES: Templates = Templates::new();
}

pub struct Templates {
    pub cache: TemplateCache<'static>,
}

impl fmt::Debug for Templates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.cache.templates.keys()).finish()
    }
}
impl Templates {
    pub fn new() -> Self {
        let cache = TemplateCache::new(&vec![]);

        Self { cache }
    }
}
