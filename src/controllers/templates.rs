#![allow(clippy::unused_async)]

use axum::debug_handler;
use axum::extract::Multipart;
use axum_extra::response::Attachment;
use cicero_dsl::compiler::compile_types;
use loco_rs::prelude::auth::JWTWithUser;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use crate::middlewares::MaybeJwtWithUser;
use crate::models::{categories, templates, users};
use crate::views::template::{CreateResponse, WithCategoriesResponse};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTemplateParams {
    pub name: String,
    pub description: String,
    pub categories: Vec<i32>,
    #[serde(flatten)]
    pub publicity: PublicityParams,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", tag = "publicity")]
pub enum PublicityParams {
    Public,
    Private { viewers: Vec<String> },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ValidateParams {
    dsl: String,
}

async fn extract_multipart(
    mut multipart: Multipart,
) -> Result<(CreateTemplateParams, Vec<u8>, String)> {
    let mut params: Option<CreateTemplateParams> = None;
    let mut docx: Option<Vec<u8>> = None;
    let mut dsl: Option<String> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| Error::BadRequest("Invalid multipart".into()))?
    {
        let name = field
            .name()
            .ok_or_else(|| Error::BadRequest("Invalid multipart".into()))?;

        match name {
            "json" => {
                let json = field
                    .text()
                    .await
                    .map_err(|_| Error::BadRequest("Invalid multipart".into()))?;
                params = Some(serde_json::from_str(&json)?);
            },
            "docx" => {
                docx =
                    Some(Vec::from(field.bytes().await.map_err(|_| {
                        Error::BadRequest("Invalid multipart".into())
                    })?));
            },
            "dsl" => {
                dsl = Some(
                    field
                        .text()
                        .await
                        .map_err(|_| Error::BadRequest("Invalid multipart".into()))?,
                );
            },
            _ => return Err(Error::BadRequest("Invalid multipart".into())),
        }
    }

    let params = params.ok_or_else(|| Error::BadRequest("Invalid multipart".into()))?;
    let docx = docx.ok_or_else(|| Error::BadRequest("Invalid multipart".into()))?;
    let dsl = dsl.ok_or_else(|| Error::BadRequest("Invalid multipart".into()))?;

    Ok((params, docx, dsl))
}

#[debug_handler]
async fn create_template(
    jwt_with_user: JWTWithUser<users::Model>,
    State(ctx): State<AppContext>,
    multipart: Multipart,
) -> Result<Response> {
    let (params, docx, dsl) = extract_multipart(multipart).await?;

    let template = templates::Model::create(
        &ctx.db,
        &params,
        jwt_with_user.user.id,
        docx.as_slice(),
        dsl.as_str(),
    )
    .await?;

    let response = CreateResponse::new(&template);

    format::json(response)
}

#[debug_handler]
async fn update_template(
    jwt_with_user: JWTWithUser<users::Model>,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    multipart: Multipart,
) -> Result<Response> {
    let (params, docx, dsl) = extract_multipart(multipart).await?;

    let template = templates::Model::find_by_id_for_user(&ctx.db, id, jwt_with_user.user.id)
        .await?
        .into_active_model()
        .update_template(
            &ctx.db,
            &params,
            id,
            jwt_with_user.user.id,
            docx.as_slice(),
            dsl.as_str(),
        )
        .await?;

    let author = users::Model::find_template_author(&ctx.db, template.user_id).await?;
    let categories = categories::Model::find_for_template(&ctx.db, id).await?;
    let viewers = users::Model::find_template_viewers(&ctx.db, id).await?;

    let response = WithCategoriesResponse::new(&template, &author, &categories, viewers.as_ref());

    format::json(response)
}

#[debug_handler]
async fn validate(
    _jwt_with_user: JWTWithUser<users::Model>,
    State(_ctx): State<AppContext>,
    mut multipart: Multipart,
) -> Result<Response> {
    if let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| Error::BadRequest("Invalid multipart".into()))?
    {
        let name = field
            .name()
            .ok_or_else(|| Error::BadRequest("Invalid multipart".into()))?;

        if name != "dsl" {
            return Err(Error::BadRequest("Invalid multipart".into()));
        }

        let dsl = field
            .text()
            .await
            .map_err(|_| Error::BadRequest("Invalid multipart".into()))?;

        let types = match compile_types(dsl.as_str()) {
            Ok(types) => types,
            Err(err) => return Err(Error::BadRequest(err)),
        };

        return format::json(types);
    };

    Err(Error::BadRequest("Invalid multipart".into()))
}

#[debug_handler]
async fn get_visible(
    MaybeJwtWithUser(maybe_jwt): MaybeJwtWithUser<users::Model>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let maybe_user_id = maybe_jwt.map(|jwt| jwt.user.id);

    let templates = templates::Model::find_visible(&ctx.db, maybe_user_id).await?;

    let mut response = Vec::with_capacity(templates.len());

    for template in templates {
        let author = users::Model::find_template_author(&ctx.db, template.user_id).await?;
        let categories = categories::Model::find_for_template(&ctx.db, template.id).await?;
        let viewers = users::Model::find_template_viewers(&ctx.db, template.id).await?;

        response.push(WithCategoriesResponse::new(
            &template,
            &author,
            &categories,
            viewers.as_ref(),
        ));
    }

    format::json(response)
}

#[debug_handler]
async fn get_one(
    MaybeJwtWithUser(maybe_jwt): MaybeJwtWithUser<users::Model>,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let maybe_user_id = maybe_jwt.map(|jwt| jwt.user.id);

    let template = templates::Model::find_visible_by_id(&ctx.db, id, maybe_user_id).await?;
    let author = users::Model::find_template_author(&ctx.db, template.user_id).await?;
    let categories = categories::Model::find_for_template(&ctx.db, id).await?;
    let viewers = users::Model::find_template_viewers(&ctx.db, id).await?;

    let response = WithCategoriesResponse::new(&template, &author, &categories, viewers.as_ref());

    format::json(response)
}

#[debug_handler]
async fn get_docx(
    MaybeJwtWithUser(maybe_jwt): MaybeJwtWithUser<users::Model>,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<impl IntoResponse> {
    let maybe_user_id = maybe_jwt.map(|jwt| jwt.user.id);
    let template = templates::Model::find_visible_by_id(&ctx.db, id, maybe_user_id).await?;

    let docx = templates::Model::find_docx(template.id).await?;

    let response = Attachment::new(docx)
        .filename(format!("{}.docx", template.id))
        .content_type("application/vnd.openxmlformats-officedocument.wordprocessingml.document");

    Ok(response)
}

#[debug_handler]
async fn get_dsl(
    MaybeJwtWithUser(maybe_jwt): MaybeJwtWithUser<users::Model>,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<impl IntoResponse> {
    let maybe_user_id = maybe_jwt.map(|jwt| jwt.user.id);
    let template = templates::Model::find_visible_by_id(&ctx.db, id, maybe_user_id).await?;

    let dsl = templates::Model::find_dsl(template.id).await?;

    let response = Attachment::new(dsl)
        .filename(format!("{}.dsl", template.id))
        .content_type("text/plain;encoding=utf-8");

    Ok(response)
}

#[debug_handler]
async fn get_dsl_types(
    MaybeJwtWithUser(maybe_jwt_with_user): MaybeJwtWithUser<users::Model>,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let maybe_user_id = maybe_jwt_with_user.map(|jwt| jwt.user.id);

    let template = templates::Model::find_visible_by_id(&ctx.db, id, maybe_user_id).await?;
    let dsl = templates::Model::find_dsl(template.id).await?;

    let types = compile_types(dsl.as_str()).map_err(Error::BadRequest)?;

    format::json(types.into_values().collect::<Vec<_>>())
}

#[debug_handler]
async fn delete_template(
    jwt_with_user: JWTWithUser<users::Model>,
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    templates::Model::delete_template(&ctx.db, id, jwt_with_user.user.id).await?;

    format::json(())
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/templates")
        .add("/", get(get_visible))
        .add("/", post(create_template))
        .add("/:id", put(update_template))
        .add("/:id", get(get_one))
        .add("/:id", delete(delete_template))
        .add("/:id/docx", get(get_docx))
        .add("/:id/dsl", get(get_dsl))
        .add("/:id/dsl/types", get(get_dsl_types))
        .add("/validate", post(validate))
}
