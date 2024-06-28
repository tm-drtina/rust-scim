use reqwest::{Client, StatusCode};
use scim_protocol::protocol::{ListResponse, ScimEndpoint, ScimResponse as _};
use scim_protocol::resource::resource_type::{self, ResourceType};
use scim_protocol::resource::schema::{self, Schema};
use scim_protocol::resource::service_provider_config::{self, ServiceProviderConfig};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::error::{Result, ScimError};

pub struct ScimClient {
    config: ScimConfig,
    http_client: Client,
}

pub struct ScimConfig {
    pub base_url: String,
    pub validate_schema: bool,
}

impl ScimClient {
    #[must_use]
    pub fn new(config: ScimConfig, http_client: Client) -> Self {
        Self {
            config,
            http_client,
        }
    }

    pub async fn get_all<T>(&self) -> Result<ListResponse<T::Response>>
    where
        T: ScimEndpoint,
        T::Response: DeserializeOwned,
    {
        let request = self.http_client.get(T::resource_uri(&self.config.base_url));
        let response = request.send().await?.error_for_status()?;
        let resources: ListResponse<T::Response> = response.json().await?;

        if self.config.validate_schema {
            for resource in &resources.resources {
                resource.validate_schema()?;
            }
        }

        Ok(resources)
    }

    pub async fn get_fresh<T>(&self, item: T::Response) -> Result<T::Response>
    where
        T: ScimEndpoint,
        T::Response: DeserializeOwned,
    {
        let mut request = self
            .http_client
            .get(T::single_resource_uri(&self.config.base_url, item.id()));

        if let Some(version) = &item.meta().version {
            request = request.header("If-None-Match", version);
        }

        let response = request.send().await?;

        match response.status() {
            StatusCode::OK => response.json().await.map_err(Into::into),
            StatusCode::NOT_MODIFIED => Ok(item),
            _ => match response.error_for_status() {
                Ok(_) => Err(ScimError::Unknown),
                Err(e) => Err(e.into()),
            },
        }
    }

    pub async fn get<T>(&self, id: &str) -> Result<T::Response>
    where
        T: ScimEndpoint,
        T::Response: DeserializeOwned,
    {
        let response = self
            .http_client
            .get(T::single_resource_uri(&self.config.base_url, id))
            .send()
            .await?
            .error_for_status()?;

        response.json().await.map_err(Into::into)
    }

    pub async fn get_service_provider_config(&self) -> Result<ServiceProviderConfig> {
        self.http_client
            .get(format!(
                "{}{}",
                &self.config.base_url,
                service_provider_config::ENDPOINT
            ))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
            .map_err(Into::into)
    }

    pub async fn get_schemas(&self) -> Result<ListResponse<Schema>> {
        self.http_client
            .get(format!("{}{}", &self.config.base_url, schema::ENDPOINT))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
            .map_err(Into::into)
    }

    pub async fn get_resource_types(&self) -> Result<ListResponse<ResourceType>> {
        self.http_client
            .get(format!(
                "{}{}",
                &self.config.base_url,
                resource_type::ENDPOINT
            ))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
            .map_err(Into::into)
    }

    pub async fn create<T>(&self, item: &T::Request) -> Result<T::Response>
    where
        T: ScimEndpoint,
        T::Request: Serialize,
        T::Response: DeserializeOwned,
    {
        let response = self
            .http_client
            .post(T::resource_uri(&self.config.base_url))
            .json(item)
            .send()
            .await?
            .error_for_status()?;

        response.json().await.map_err(Into::into)
    }

    pub async fn delete<T: ScimEndpoint>(&self, item: &T::Response) -> Result<()> {
        let mut request = self
            .http_client
            .delete(T::single_resource_uri(&self.config.base_url, item.id()));

        if let Some(version) = &item.meta().version {
            request = request.header("If-Match", version);
        }

        let _response = request.send().await?.error_for_status()?;
        Ok(())
    }

    pub async fn delete_by_id<T>(&self, id: &str, version: Option<&str>) -> Result<()>
    where
        T: ScimEndpoint,
    {
        let mut request = self
            .http_client
            .delete(T::single_resource_uri(&self.config.base_url, id));

        if let Some(version) = version {
            request = request.header("If-Match", version);
        }

        let _response = request.send().await?.error_for_status()?;

        Ok(())
    }
}
