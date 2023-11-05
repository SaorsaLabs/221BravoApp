<script>
    import distriktLogo from '$lib/images/distrikt.png';
    import ytLogo from '$lib/images/youtube.png';
    import twitter from '$lib/images/twitter.png';
    import icLogo from '$lib/images/InternetComputer.png';
</script>
<footer class="foot">
    <div class="container-lg">
        <div class="row" style="padding-bottom: 15px; padding-top: 5px">
            <div class="col alignLeft">
                ©️ Copyright Saorsa Labs 2023
                {@html "<br/>"}
                <span class="terms">
                    IMPORTANT: All information can be inaccurate, {@html "<br>"}
                    please read our <a href="/misc/disclaimer/">Terms and Conditions</a>
                </span>
            </div>
            <div class="col alignCentre">
                Powered By
                {@html "<br>"}
                <a href="https://internetcomputer.org/" target="_blank" style="padding-bottom: 10px;"><img src={icLogo} alt="Internet Computer"/></a>
            </div>
            <div class="col alignRight">
                <div style="padding-right: 20px;">
                    <a href="https://distrikt.app/u/SaorsaLabs" target="_blank" style="padding: 5px;"><img src={distriktLogo} alt="Distrikt" /></a>
                    <a href="https://www.youtube.com/channel/UCzs4AfPOtZXvX41c9qW8ecw" target="_blank" style="padding: 5px;"><img src={ytLogo} alt="You Tube" /></a>
                    <a href="https://twitter.com/221BravoApp" target="_blank" style="padding: 5px;"><img src={twitter} alt="Twitter" /></a>
                </div>
            </div>
        </div>
    </div>
</footer>

<style>
    .pad{
        padding: 5px;
    }
    .foot{
        flex: auto;
        background-color: rgb(0, 0, 0);
         min-width: 700px;
    }
    .alignCentre{
        text-align: center;
        color: rgba(255, 255, 255, 0.605);
    }
    .alignLeft{
        text-align: left;
        color: rgba(255, 255, 255, 0.605);
    }
    .alignRight{
        text-align: right;
        color: rgba(255, 255, 255, 0.605);
    }
    .terms{
        text-align: right;
        color: rgba(255, 255, 255, 0.605);
        font-size: smaller;
    }
    a{
        color: azure;
    }
</style>