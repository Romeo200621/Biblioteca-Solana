use anchor_lang::prelude::*;

// ID del Solana Program
declare_id!("");

#[program]
pub mod bar {
    use super::*;

    //////////////////////////// Instruccion: Crear Bar /////////////////////////////////////
    /*
    Permite crear una cuenta PDA que almacenará el Bar y sus bebidas.
    */

    pub fn crear_bar(context: Context<NuevoBar>, nombre: String) -> Result<()> {
        let owner_id = context.accounts.owner.key();
        msg!("Owner id: {}", owner_id);

        let bebidas: Vec<Bebida> = Vec::new();

        context.accounts.bar.set_inner(Bar {
            owner: owner_id,
            nombre,
            bebidas,
        });

        Ok(())
    }

    //////////////////////////// Instruccion: Agregar Bebida /////////////////////////////////////
    /*
    Permite agregar una bebida al menú del bar
    */

    pub fn agregar_bebida(context: Context<NuevaBebida>, nombre: String, precio: u16) -> Result<()> {
        require!(
            context.accounts.bar.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let bebida = Bebida {
            nombre,
            precio,
            disponible: true,
        };

        context.accounts.bar.bebidas.push(bebida);

        Ok(())
    }

    //////////////////////////// Instruccion: Eliminar Bebida /////////////////////////////////////
    /*
    Elimina una bebida del menú
    */

    pub fn eliminar_bebida(context: Context<NuevaBebida>, nombre: String) -> Result<()> {
        require!(
            context.accounts.bar.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let bebidas = &mut context.accounts.bar.bebidas;

        for i in 0..bebidas.len() {
            if bebidas[i].nombre == nombre {
                bebidas.remove(i);
                msg!("Bebida {} eliminada!", nombre);
                return Ok(());
            }
        }

        Err(Errores::BebidaNoExiste.into())
    }

    //////////////////////////// Instruccion: Ver Bebidas /////////////////////////////////////

    pub fn ver_bebidas(context: Context<NuevaBebida>) -> Result<()> {
        require!(
            context.accounts.bar.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        msg!(
            "La lista de bebidas disponibles es: {:#?}",
            context.accounts.bar.bebidas
        );

        Ok(())
    }

    //////////////////////////// Instruccion: Alternar Disponibilidad /////////////////////////////////////

    pub fn alternar_disponibilidad(context: Context<NuevaBebida>, nombre: String) -> Result<()> {
        require!(
            context.accounts.bar.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let bebidas = &mut context.accounts.bar.bebidas;

        for i in 0..bebidas.len() {
            let estado = bebidas[i].disponible;

            if bebidas[i].nombre == nombre {
                let nuevo_estado = !estado;
                bebidas[i].disponible = nuevo_estado;

                msg!(
                    "La bebida: {} ahora tiene disponibilidad: {}",
                    nombre,
                    nuevo_estado
                );

                return Ok(());
            }
        }

        Err(Errores::BebidaNoExiste.into())
    }
}

/////////////////////////////////////////////////////////

#[error_code]
pub enum Errores {
    #[msg("Error, no eres el propietario del bar")]
    NoEresElOwner,

    #[msg("Error, la bebida no existe")]
    BebidaNoExiste,
}

/////////////////////////////////////////////////////////

#[account]
#[derive(InitSpace)]
pub struct Bar {
    owner: Pubkey,

    #[max_len(60)]
    nombre: String,

    #[max_len(10)]
    bebidas: Vec<Bebida>,
}

/////////////////////////////////////////////////////////

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Bebida {
    #[max_len(60)]
    nombre: String,

    precio: u16,

    disponible: bool,
}

/////////////////////////////////////////////////////////

#[derive(Accounts)]
pub struct NuevoBar<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = Bar::INIT_SPACE + 8,
        seeds = [b"bar", owner.key().as_ref()],
        bump
    )]
    pub bar: Account<'info, Bar>,

    pub system_program: Program<'info, System>,
}

/////////////////////////////////////////////////////////

#[derive(Accounts)]
pub struct NuevaBebida<'info> {
    pub owner: Signer<'info>,

    #[account(mut)]
    pub bar: Account<'info, Bar>,
}
