use anyhow::{bail, Context, Result};
use std::{env, ffi::OsString, fs, str::FromStr};

/// The main setup function
fn main() -> Result<()>
{
    println!( " ==================================================================== " );
    println!( " ==================================================================== " );
    println!( "  ██████   █████  ██████  ██████  ██   ██ ███████ ███████ ██  █████   " );
    println!( "  ██   ██ ██   ██ ██   ██ ██   ██ ██   ██ ██      ██      ██ ██   ██  " );
    println!( "  ██████  ███████ █████   █████   ███████ █████   ███████ ██ ███████  " );
    println!( "  ██      ██   ██ ██  ██  ██  ██  ██   ██ ██           ██ ██ ██   ██  " );
    println!( "  ██      ██   ██ ██   ██ ██   ██ ██   ██ ███████ ███████ ██ ██   ██  " );
    println!( "  V0.1.0              Shared with much love by Joey             2022  " );
    println!( " ==================================================================== " );
    println!( " ==================================================================== " );
    println!( "                             INTRODUCTION                             " );
    println!( " Welcome to the Parrhesia setup script! This is a simple utility that " );
    println!( " performs first-time initialization for the project and moves the     " );
    println!( " Gizmo utility into the root project directory.                       " );
    println!( "                                                                      " );
    println!( "                                GIZMO                                 " );
    println!( " Gizmo is a fully native autocoding / project management tool that    " );
    println!( " makes your life easier. Invoke it with no arguments to see the usage " );
    println!( " guide, or visit https://link_to_be_made_later.                       " );
    println!( "                                                                      " );
    println!( "                         INSTALL (TEMPORARY)                          " );
    println!( " To use Gizmo without typing the path to it, run this command:        " );

    let mut exe_dir = env::current_exe().with_context( || "Unable to determine location of target output. Please report this error, as it shouldn't be possible to reach" )?;

    let gizmo_target = 'target: {
        exe_dir.pop();

        for path in fs::read_dir(&exe_dir).with_context( || format!( "Failed to get file list from {:?}", exe_dir ) )?
        {
            let Ok( path ) = path else {
                println!( "Failed to process the following directory entry: {:?}", path );
                continue;
            };
            let name = path.file_name();

            if name != OsString::from_str("gizmo.d").unwrap() // This unwrap can't be reached since the string has to be valid, since it exists
                && name.to_str().unwrap().contains("gizmo")
            {
                let Some( name_str ) = name.to_str() else {
                    println!( "File name '{:?}' contains invalid unicode!", name );
                    continue;
                };

                if name_str.contains( "gizmo" ) {
                    break 'target path;
                }
            }
        }

        bail!("Failed to find the gizmo executable in {:?}! Try rebuilding?", exe_dir);
    }.path();

    let project_gizmo = {
        exe_dir.pop();
        exe_dir.pop();
        exe_dir.push(gizmo_target.file_name().unwrap());
        exe_dir
    };

    if cfg!(windows) {
        println!( "  - doskey gizmo={:?} $*", project_gizmo );
    } else {
        println!( "  - alias gizmo={:?}", project_gizmo );
    }

    let copy = if project_gizmo.exists() {
        let btime = gizmo_target
            .metadata().with_context( || format!( "Failed to get metadata of the following file: {:?}", gizmo_target ) )?
            .modified().with_context( || format!( "Failed to get modified time of the following file: {:?}", gizmo_target ) )?;
        
        let ptime = project_gizmo
            .metadata().with_context( || format!( "Failed to get metadata of the following file: {:?}", project_gizmo ) )?
            .modified().with_context( || format!( "Failed to get modified time of the following file: {:?}", project_gizmo ) )?;

        if btime > ptime {
            true
        } else {
            false
        }
    } else {
        true
    };

    if copy {
        fs::copy( &gizmo_target, &project_gizmo ).with_context( || format!( "Failed to copy {:?} to {:?}", gizmo_target, project_gizmo ) )?;
        println!( "Moved gizmo to the project directory" );
    }

    println!( "                                                                      " );
    println!( "                         INSTALL (PERMANENT)                          " );
    println!( " To enable the use of Gizmo across terminal sessions, you will have to" );
    println!( " add the project folder to your PATH. This process depends on your    " );
    println!( " specific environment, so look for instructions elsewhere.            " );
    println!( "   - {:?}", project_gizmo.parent().unwrap() );

    Ok(())
}
