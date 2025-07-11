use std::collections::HashSet;

/// Models the Permissions of each node.
/// Permissions are **transitive**. If node **n** has Permissions **P** all its childrens
/// Permissions **P_1 ... P_n** <= **P**.
///
/// Admin : Everything
/// User : Everything other than Admin and Groups
/// Groups: Only its group. If node has Group a and node Group a and b access is **Granted**.
/// Public: Only public nodes.
#[derive(Clone)]
pub enum Permissions {
    Admin,
    User(Option<Vec<String>>), // option of possible groups.
    Public,
}

impl Permissions {
    /// Returns true iff the [other] permissions are more powerfull then [self]
    pub fn can_be_accessed(&self, other: &Permissions) -> bool {
        match (self, other) {
            (_, Permissions::Admin) => true,  // Admins acces everything
            (Permissions::Admin, _) => false, // Needed permissions are admin but not admin
            // permissions
            (Permissions::Public, _) => true, // Public can be accesed by all
            (Permissions::User(None), Permissions::User(_)) => true, // Anyone with user
            // permissions can acces a user node without any groups.
            (Permissions::User(Some(groups)), Permissions::User(Some(possible_groups))) => {
                let mut set: HashSet<&String> = HashSet::new();
                // Generate a map of the groups that the accesor has
                for group in possible_groups {
                    set.insert(group);
                }

                groups.iter().any(|v| set.contains(v))
            }
            (Permissions::User(Some(ls)), Permissions::User(None)) => ls.is_empty(), // Only ok
            // when there are no groups otherwise group permissions are missing.
            (Permissions::User(_), Permissions::Public) => false,
        }
    }
}

#[cfg(test)]
mod permissions_tests {
    use crate::security::permissions::Permissions;

    #[test]
    fn test() {
        assert!(Permissions::Public.can_be_accessed(&Permissions::Public));
        assert!(!Permissions::User(None).can_be_accessed(&Permissions::Public));
        assert!(Permissions::User(None).can_be_accessed(&Permissions::User(Some(vec![]))));
        assert!(
            !Permissions::User(Some(vec!["test".to_string()]))
                .can_be_accessed(&Permissions::User(Some(vec![])))
        );
        assert!(
            Permissions::User(Some(vec!["test".to_string()])).can_be_accessed(&Permissions::User(
                Some(vec!["test".to_string(), "test2".to_string()])
            ))
        );
        assert!(
            !Permissions::User(Some(vec!["test3".to_string()])).can_be_accessed(
                &Permissions::User(Some(vec!["test".to_string(), "test2".to_string()]))
            )
        );
    }
}
