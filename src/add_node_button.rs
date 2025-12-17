use leptos::prelude::*;

use crate::components::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuAction, DropdownMenuAlign, DropdownMenuContent, DropdownMenuGroup,
    DropdownMenuItem, DropdownMenuLabel, DropdownMenuLink, DropdownMenuSub, DropdownMenuSubContent,
    DropdownMenuSubItem, DropdownMenuSubTrigger, DropdownMenuTrigger,
};

#[component]
pub fn AddNodeButton() -> impl IntoView {
    view! {
        <DropdownMenu align=DropdownMenuAlign::Start>
            <DropdownMenuTrigger>"Open (Start)"</DropdownMenuTrigger>

            <DropdownMenuContent>
                <DropdownMenuLabel>"Start Menu"</DropdownMenuLabel>

                <DropdownMenuGroup>
                    <DropdownMenuItem>
                        <DropdownMenuAction>"Simple Item"</DropdownMenuAction>
                    </DropdownMenuItem>

                    <DropdownMenuSub>
                        <DropdownMenuSubTrigger>"Settings"</DropdownMenuSubTrigger>
                        <DropdownMenuSubContent>
                            <DropdownMenuSubItem>"Account Settings"</DropdownMenuSubItem>
                            <DropdownMenuSubItem>"Privacy Settings"</DropdownMenuSubItem>
                            <DropdownMenuSubItem>"Notification Settings"</DropdownMenuSubItem>
                        </DropdownMenuSubContent>
                    </DropdownMenuSub>

                    <DropdownMenuSub>
                        <DropdownMenuSubTrigger>"Tools"</DropdownMenuSubTrigger>
                        <DropdownMenuSubContent>
                            <DropdownMenuSubItem>"Export Data"</DropdownMenuSubItem>
                            <DropdownMenuSubItem>"Import Data"</DropdownMenuSubItem>
                            <DropdownMenuSubItem>"Developer Tools"</DropdownMenuSubItem>
                        </DropdownMenuSubContent>
                    </DropdownMenuSub>
                </DropdownMenuGroup>

                <DropdownMenuGroup>
                    <DropdownMenuItem>
                        <DropdownMenuLink attr:href="/">"Home"</DropdownMenuLink>
                    </DropdownMenuItem>
                    <DropdownMenuItem>
                        <DropdownMenuAction>"Sign Out"</DropdownMenuAction>
                    </DropdownMenuItem>
                </DropdownMenuGroup>
            </DropdownMenuContent>
        </DropdownMenu>
    }
}
