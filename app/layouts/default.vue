<template>
    <link href="https://fonts.googleapis.com/css2?family=Montserrat&display=swap" rel="stylesheet"> 

    <Navbar/>

    <div class="center-box">
        <main>
            <slot/>
        </main>
    </div>
</template>

<style lang="sass">
// I wanted to put this in `global.sass`, but for some reason, if I do that, the whole
// thing explodes
*
    padding: 0
    margin: 0

.center-box
    display: flex
    justify-content: center
    align-items: center

main
    width: 800px
    margin-top: $small-margin
</style>