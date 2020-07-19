trait User {
    fn new(username: &'static str) -> Self;
    // Método chamado na inicialização de uma instância que implementa o trait

    fn username(&self) -> &'static str;

    fn login(&self) -> &'static str;

    fn logout(&self) -> &'static str;

    fn is_logged_in(&self) -> bool {
        false
    }
}

struct Admin {
    username: &'static str,
}

struct Operador {
    username: &'static str,
}

struct BasicUser {
    username: &'static str,
}

impl User for Admin {
    fn new(username: &'static str) -> Admin {
        Admin { username }
    }

    fn username(&self) -> &'static str {
        self.username
    }

    fn login(&self) -> &'static str {
        "Usuário do tipo ADMIN entrou no sistema"
    }

    fn logout(&self) -> &'static str {
        "Usuário do tipo ADMIN saiu no sistema"
    }
}

impl User for Operador {
    fn new(username: &'static str) -> Operador {
        Operador { username }
    }

    fn username(&self) -> &'static str {
        self.username
    }

    fn login(&self) -> &'static str {
        "Usuário do tipo Operador entrou no sistema"
    }

    fn logout(&self) -> &'static str {
        "Usuário do tipo Operador saiu no sistema"
    }
}

impl User for BasicUser {
    fn new(username: &'static str) -> BasicUser {
        BasicUser { username }
    }

    fn username(&self) -> &'static str {
        self.username
    }

    fn login(&self) -> &'static str {
        "Usuário do tipo BasicUser entrou no sistema"
    }

    fn logout(&self) -> &'static str {
        "Usuário do tipo BasicUser saiu no sistema"
    }
}

fn main() {
    let admin: Admin = User::new("Ilgsson");
    let operador: Operador = User::new("Arlaine");
    let basic_user: BasicUser = User::new("Ilgner");

    println!("Seja bem vindo, {}!", admin.username());
    println!("{}", admin.login());
    println!("{}", admin.logout());

    println!("Seja bem vindo, {}!", operador.username());
    println!("{}", operador.login());
    println!("{}", operador.logout());

    println!("Seja bem vindo, {}!", basic_user.username());
    println!("{}", basic_user.login());
    println!("{}", basic_user.logout());
}
