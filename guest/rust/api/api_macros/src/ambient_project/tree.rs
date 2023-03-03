use std::collections::BTreeMap;

use super::{
    identifier::{Identifier, IdentifierPath, IdentifierPathBuf},
    manifest::{Component, Concept, Manifest, Namespace, NamespaceOrOther},
};
use quote::quote;

pub(super) trait Tree {
    type Item;
    fn new(mainfest: &Manifest, validate_namespaces_documented: bool) -> anyhow::Result<Self>
    where
        Self: Sized;

    fn to_token_stream(
        &self,
        api_name: &syn::Path,
        project_path: IdentifierPath,
    ) -> anyhow::Result<proc_macro2::TokenStream>;

    fn get(&self, path: IdentifierPath) -> Option<&Self::Item>;

    fn insert (&mut self,  path: IdentifierPathBuf, inner: TreeNodeInner<Self::Item>)
        -> anyhow::Result<()>;
}
pub(super) trait TreeNode {
    type Item;
    fn new(path: IdentifierPathBuf, inner: TreeNodeInner<Self::Item>) -> Self
    where Self: Sized;


    fn to_token_stream(
        &self,
        api_name: &syn::Path,
        project_path: IdentifierPath,
    ) -> anyhow::Result<proc_macro2::TokenStream>;
}


pub(super) trait TreeNodeNamespace : TreeNode{
    fn new(namespace: Option<Namespace>) -> Self
    where
        Self: Sized;
    fn get(&self, path: IdentifierPath) -> Option<&Self::Item>;
}


 /* *
#[derive(Debug, Clone)]
pub(super) struct TreeNodeNamespace
{
    pub(super) children: BTreeMap<Identifier, dyn TreeNode>,
    pub(super) namespace: Option<Namespace>,
}



impl  TreeNodeNamespace{
    pub(super) fn new(namespace: Option<Namespace>) -> Self
    {
        Self {
            children: BTreeMap::new(),
            namespace,
        }
    }
    pub(super) fn get<Y>(&self, path: IdentifierPath) -> Option<&Y> {
        let (root, rest) = path.split_first()?;
        let child = self.children.get(root)?;
        match &child.inner {
            TreeNodeInner::Namespace(ns) => ns.get(IdentifierPath(rest)),
            TreeNodeInner::Other(c) => Some(c),
        }
    }
}
*/


#[derive(Debug)]
pub(super) enum TreeNodeInner<T> 
{
    Namespace(Box<dyn TreeNodeNamespace<Item=T>>),
    Other(T),
}

/* 
fn ensure_namespace_documented(node: &impl TreeNode) -> anyhow::Result<()> {
    match &node.inner {
        TreeNodeInner::Namespace(TreeNodeNamespace {
            namespace: None, ..
        }) => anyhow::bail!(
            "The namespace `{}` is missing a name and description.",
            node.path
        ),
        TreeNodeInner::Namespace(TreeNodeNamespace {
            children,
            namespace: Some(_),
        }) => {
            for node in children.values() {
                ensure_namespace_documented(node)?;
            }
        }
        _ => {}
    }
    Ok(())
}

*/