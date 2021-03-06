use elsa::FrozenMap;
use fluent_bundle::{FluentBundle, FluentResource};
use fluent_fallback::generator::{BundleGenerator, FluentBundleResult};
use futures::stream::Stream;
use std::fs;
use std::io;
use std::iter;
use unic_langid::LanguageIdentifier;

fn read_file(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

pub struct ResourceManager {
    resources: FrozenMap<String, Box<FluentResource>>,
    path_scheme: String,
}

impl ResourceManager {
    pub fn new(path_scheme: String) -> Self {
        ResourceManager {
            resources: FrozenMap::new(),
            path_scheme,
        }
    }

    fn get_resource(&self, res_id: &str, locale: &str) -> &FluentResource {
        let path = self
            .path_scheme
            .replace("{locale}", locale)
            .replace("{res_id}", res_id);
        if let Some(res) = self.resources.get(&path) {
            res
        } else {
            let string = read_file(&path).unwrap();
            let res = match FluentResource::try_new(string) {
                Ok(res) => res,
                Err((res, _err)) => res,
            };
            self.resources.insert(path.to_string(), Box::new(res))
        }
    }

    pub fn get_bundle(
        &self,
        locales: Vec<LanguageIdentifier>,
        resource_ids: Vec<String>,
    ) -> FluentBundle<&FluentResource> {
        let mut bundle = FluentBundle::new(locales.clone());
        for res_id in &resource_ids {
            let res = self.get_resource(res_id, &locales[0].to_string());
            bundle.add_resource(res).unwrap();
        }
        bundle
    }

    pub fn get_bundles<'l>(
        &'l self,
        locales: Vec<LanguageIdentifier>,
        resource_ids: Vec<String>,
    ) -> impl Iterator<Item = FluentBundle<&FluentResource>> {
        let res_mgr = self;
        let mut ptr = 0;

        iter::from_fn(move || {
            locales.get(ptr).map(|locale| {
                ptr += 1;
                let mut bundle = FluentBundle::new(vec![locale.clone()]);
                for res_id in &resource_ids {
                    let res = res_mgr.get_resource(&res_id, &locale.to_string());
                    bundle.add_resource(res).unwrap();
                }
                bundle
            })
        })
    }
}

// Due to limitation of trait, we need a nameable Iterator type.  Due to the
// lack of GATs, these have to own members instead of taking slices.
pub struct BundleIter {
    locales: <Vec<LanguageIdentifier> as IntoIterator>::IntoIter,
    resource_ids: Vec<String>,
}

impl Iterator for BundleIter {
    type Item = FluentBundleResult<FluentResource>;

    fn next(&mut self) -> Option<Self::Item> {
        let locale = self.locales.next()?;

        let mut bundle = FluentBundle::new(vec![locale.clone()]);

        for res_id in &self.resource_ids {
            let full_path = format!("./tests/resources/{}/{}", locale, res_id);
            let source = fs::read_to_string(full_path).unwrap();
            let res = FluentResource::try_new(source).unwrap();
            bundle.add_resource(res).unwrap();
        }
        Some(Ok(bundle))
    }
}

impl Stream for BundleIter {
    type Item = FluentBundleResult<FluentResource>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        todo!()
    }
}

impl BundleGenerator for ResourceManager {
    type Resource = FluentResource;
    type Iter = BundleIter;
    type Stream = BundleIter;

    fn bundles_iter(&self, resource_ids: Vec<String>) -> Self::Iter {
        BundleIter {
            locales: vec!["en-US".parse().unwrap(), "pl".parse().unwrap()].into_iter(),
            resource_ids,
        }
    }

    fn bundles_stream(&self, _res_ids: Vec<String>) -> Self::Stream {
        todo!()
    }
}
