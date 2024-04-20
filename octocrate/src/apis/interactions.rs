use octocrate_core::*;
#[allow(unused_imports)]
use octocrate_types::*;

/// Owner or admin management of users interactions.
pub struct GitHubInteractionsAPI {
  config: SharedAPIConfig,
}

impl GitHubInteractionsAPI {
  pub fn new(config: &SharedAPIConfig) -> Self {
    Self {
      config: config.clone(),
    }
  }

  /// **Get interaction restrictions for a repository**
  ///
  /// Shows which type of GitHub user can interact with this repository and when the restriction expires. If there are no restrictions, you will see an empty response.
  ///
  /// *Documentation*: [https://docs.github.com/rest/interactions/repos#get-interaction-restrictions-for-a-repository](https://docs.github.com/rest/interactions/repos#get-interaction-restrictions-for-a-repository)
  pub fn get_restrictions_for_repo(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<(), (), InteractionsGetRestrictionsForRepoResponse> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/interaction-limits");

    Request::<(), (), InteractionsGetRestrictionsForRepoResponse>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Set interaction restrictions for a repository**
  ///
  /// Temporarily restricts interactions to a certain type of GitHub user within the given repository. You must have owner or admin access to set these restrictions. If an interaction limit is set for the user or organization that owns this repository, you will receive a `409 Conflict` response and will not be able to use this endpoint to change the interaction limit for a single repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/interactions/repos#set-interaction-restrictions-for-a-repository](https://docs.github.com/rest/interactions/repos#set-interaction-restrictions-for-a-repository)
  pub fn set_restrictions_for_repo(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> Request<InteractionRestrictions, (), InteractionLimits> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/interaction-limits");

    Request::<InteractionRestrictions, (), InteractionLimits>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Remove interaction restrictions for a repository**
  ///
  /// Removes all interaction restrictions from the given repository. You must have owner or admin access to remove restrictions. If the interaction limit is set for the user or organization that owns this repository, you will receive a `409 Conflict` response and will not be able to use this endpoint to change the interaction limit for a single repository.
  ///
  /// *Documentation*: [https://docs.github.com/rest/interactions/repos#remove-interaction-restrictions-for-a-repository](https://docs.github.com/rest/interactions/repos#remove-interaction-restrictions-for-a-repository)
  pub fn remove_restrictions_for_repo(
    &self,
    owner: impl Into<String>,
    repo: impl Into<String>,
  ) -> NoContentRequest<(), ()> {
    let owner = owner.into();
    let repo = repo.into();
    let url = format!("/repos/{owner}/{repo}/interaction-limits");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Get interaction restrictions for your public repositories**
  ///
  /// Shows which type of GitHub user can interact with your public repositories and when the restriction expires.
  ///
  /// *Documentation*: [https://docs.github.com/rest/interactions/user#get-interaction-restrictions-for-your-public-repositories](https://docs.github.com/rest/interactions/user#get-interaction-restrictions-for-your-public-repositories)
  pub fn get_restrictions_for_authenticated_user(
    &self,
  ) -> Request<(), (), InteractionsGetRestrictionsForAuthenticatedUserResponse> {
    let url = format!("/user/interaction-limits");

    Request::<(), (), InteractionsGetRestrictionsForAuthenticatedUserResponse>::builder(
      &self.config,
    )
    .get(url)
    .build()
  }

  /// **Set interaction restrictions for your public repositories**
  ///
  /// Temporarily restricts which type of GitHub user can interact with your public repositories. Setting the interaction limit at the user level will overwrite any interaction limits that are set for individual repositories owned by the user.
  ///
  /// *Documentation*: [https://docs.github.com/rest/interactions/user#set-interaction-restrictions-for-your-public-repositories](https://docs.github.com/rest/interactions/user#set-interaction-restrictions-for-your-public-repositories)
  pub fn set_restrictions_for_authenticated_user(
    &self,
  ) -> Request<InteractionRestrictions, (), InteractionLimits> {
    let url = format!("/user/interaction-limits");

    Request::<InteractionRestrictions, (), InteractionLimits>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Remove interaction restrictions from your public repositories**
  ///
  /// Removes any interaction restrictions from your public repositories.
  ///
  /// *Documentation*: [https://docs.github.com/rest/interactions/user#remove-interaction-restrictions-from-your-public-repositories](https://docs.github.com/rest/interactions/user#remove-interaction-restrictions-from-your-public-repositories)
  pub fn remove_restrictions_for_authenticated_user(&self) -> NoContentRequest<(), ()> {
    let url = format!("/user/interaction-limits");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }

  /// **Get interaction restrictions for an organization**
  ///
  /// Shows which type of GitHub user can interact with this organization and when the restriction expires. If there is no restrictions, you will see an empty response.
  ///
  /// *Documentation*: [https://docs.github.com/rest/interactions/orgs#get-interaction-restrictions-for-an-organization](https://docs.github.com/rest/interactions/orgs#get-interaction-restrictions-for-an-organization)
  pub fn get_restrictions_for_org(
    &self,
    org: impl Into<String>,
  ) -> Request<(), (), InteractionsGetRestrictionsForOrgResponse> {
    let org = org.into();
    let url = format!("/orgs/{org}/interaction-limits");

    Request::<(), (), InteractionsGetRestrictionsForOrgResponse>::builder(&self.config)
      .get(url)
      .build()
  }

  /// **Set interaction restrictions for an organization**
  ///
  /// Temporarily restricts interactions to a certain type of GitHub user in any public repository in the given organization. You must be an organization owner to set these restrictions. Setting the interaction limit at the organization level will overwrite any interaction limits that are set for individual repositories owned by the organization.
  ///
  /// *Documentation*: [https://docs.github.com/rest/interactions/orgs#set-interaction-restrictions-for-an-organization](https://docs.github.com/rest/interactions/orgs#set-interaction-restrictions-for-an-organization)
  pub fn set_restrictions_for_org(
    &self,
    org: impl Into<String>,
  ) -> Request<InteractionRestrictions, (), InteractionLimits> {
    let org = org.into();
    let url = format!("/orgs/{org}/interaction-limits");

    Request::<InteractionRestrictions, (), InteractionLimits>::builder(&self.config)
      .put(url)
      .build()
  }

  /// **Remove interaction restrictions for an organization**
  ///
  /// Removes all interaction restrictions from public repositories in the given organization. You must be an organization owner to remove restrictions.
  ///
  /// *Documentation*: [https://docs.github.com/rest/interactions/orgs#remove-interaction-restrictions-for-an-organization](https://docs.github.com/rest/interactions/orgs#remove-interaction-restrictions-for-an-organization)
  pub fn remove_restrictions_for_org(&self, org: impl Into<String>) -> NoContentRequest<(), ()> {
    let org = org.into();
    let url = format!("/orgs/{org}/interaction-limits");

    NoContentRequest::<(), ()>::builder(&self.config)
      .delete(url)
      .build()
  }
}
