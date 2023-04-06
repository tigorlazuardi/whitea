<script lang="ts">
import { Alert } from "flowbite-svelte";
import { Navbar, NavBrand, NavLi, NavUl, NavHamburger } from "flowbite-svelte";
</script>

<main>
    <Navbar let:hidden let:toggle>
        <NavBrand href="/">
            <img
                src="https://flowbite.com/docs/images/logo.svg"
                class="mr-3 h-6 sm:h-9"
                alt="Flowbite Logo" />
            <span
                class="self-center whitespace-nowrap text-xl font-semibold dark:text-white">
                Whitea
            </span>
        </NavBrand>
        <NavHamburger on:click={toggle} />
        <NavUl hidden={hidden}>
            <NavLi href="/" active={true}>Home</NavLi>
            <NavLi href="/about">About</NavLi>
            <NavLi href="/services">Services</NavLi>
            <NavLi href="/pricing">Pricing</NavLi>
            <NavLi href="/contact">Contact</NavLi>
        </NavUl>
    </Navbar>
    <Alert>
        <span class="font-bold">Info alert!</span> Change a few things up and try
        submitting again.
    </Alert>
</main>
