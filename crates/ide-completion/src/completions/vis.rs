//! Completion for visibility specifiers.

use hir::ScopeDef;

use crate::{
    context::{CompletionContext, PathCompletionCtx, PathKind, PathQualifierCtx},
    Completions,
};

pub(crate) fn complete_vis_path(acc: &mut Completions, ctx: &CompletionContext) {
    let (&is_absolute_path, qualifier, &has_in_token) = match ctx.path_context() {
        Some(PathCompletionCtx {
            kind: PathKind::Vis { has_in_token },
            is_absolute_path,
            qualifier,
            ..
        }) => (is_absolute_path, qualifier, has_in_token),
        _ => return,
    };

    match qualifier {
        Some(PathQualifierCtx { resolution, is_super_chain, .. }) => {
            // Try completing next child module of the path that is still a parent of the current module
            if let Some(hir::PathResolution::Def(hir::ModuleDef::Module(module))) = resolution {
                let next_towards_current = ctx
                    .module
                    .path_to_root(ctx.db)
                    .into_iter()
                    .take_while(|it| it != module)
                    .last();
                if let Some(next) = next_towards_current {
                    if let Some(name) = next.name(ctx.db) {
                        cov_mark::hit!(visibility_qualified);
                        acc.add_resolution(ctx, name, ScopeDef::ModuleDef(next.into()));
                    }
                }
            }

            if *is_super_chain {
                acc.add_keyword(ctx, "super::");
            }
        }
        None if !is_absolute_path => {
            if !has_in_token {
                cov_mark::hit!(kw_completion_in);
                acc.add_keyword(ctx, "in");
            }
            acc.add_nameref_keywords(ctx);
        }
        _ => {}
    }
}
