#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Section {
    Dashboard,
    Articles,
    Media,
    Contributions,
    Users,
}

impl Section {
    pub fn short_name(self) -> &'static str {
        match self {
            Self::Dashboard => "Tableau de bord",
            Self::Articles => "Articles",
            Self::Media => "Fichiers",
            Self::Contributions => "Contributions",
            Self::Users => "Utilisateurs",
        }
    }

    pub fn long_name(self) -> &'static str {
        match self {
            Self::Media => "Fichiers mis en ligne",
            Self::Contributions => "Contributions en attente",
            Self::Users => "Gestion des utilisateur·ice·s",
            o => o.short_name(),
        }
    }

    pub fn selected(self, o: Self) -> &'static str {
        if self == o {
            " selected"
        } else {
            ""
        }
    }
}
