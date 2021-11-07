import { LitElement, html, css, customElement, property, query, PropertyValues } from "lit-element";
import "@elements/core/images/ui";

export type StateName = "unstarted" | "ended" | "playing" | "paused" | "buffering" | "cued";

declare global {
    interface Window {
        youTubeEvents?: YouTubeEvents;
        onYouTubeIframeAPIReady?: () => void;
    }
}

@customElement("video-youtube-player")
export class _ extends LitElement {
    static get styles() {
        return [
            css`
                :host {
                    display: block;
                    width: 480rem;
                    height: 270rem;
                    background-color: black;
                }
                iframe {
                    height: 100%;
                    width: 100%;
                }
            `,
        ];
    }

    @query("#container")
    container!: HTMLElement;

    @property()
    videoId?: string;

    @property({ type: Boolean })
    autoplay: boolean = false;

    @property({ type: Boolean })
    loop: boolean = false;

    @property({ type: Boolean })
    captions: boolean = false;

    @property({ type: Boolean })
    muted: boolean = false;

    @property({ type: Boolean })
    hideControls: boolean = false;

    @property({ type: Number })
    start?: number;

    @property({ type: Number })
    end?: number;

    private player?: YT.Player;

    public get playerState(): StateName {
        return this.stateName(this.player!.getPlayerState());
    }

    firstUpdated() {
        if(!window.youTubeEvents) {
            window.youTubeEvents = new YouTubeEvents;
        }

        if(window.youTubeEvents.apiReady) {
            this.init();
        } else {
            window.youTubeEvents.addEventListener("api-ready", this.init, {once: true});
        }

        this.setupOnEnded();
    }

    updated(changedProperties: PropertyValues) {
        if (changedProperties.has('muted')) {
            this.mutedChanged();
        }
        if (changedProperties.has('loop')) {
            this.loopChanged();
        }
        if (changedProperties.has('autoplay')) {
            this.autoplayChanged();
        }
        if (changedProperties.has('captions')) {
            console.log("TODO: captions change");
        }
    }

    private setupOnEnded() {
        this.addEventListener("youtube-ended", () => {
            if(this.loop)
                this.play();
        });
    }

    private mutedChanged() {
        if(this.muted)
            this.player?.mute();
        else
            this.player?.unMute();
    }
    private loopChanged() {
        this.player?.setLoop(this.loop);
    }
    private autoplayChanged() {
        if(this.autoplay)
            this.player?.playVideo();
        // TODO: what if the video is not loaded yet? Have to deal with this one
    }

    public play() {
        this.player?.playVideo();
    }

    private init = () => {
        this.player = new YT.Player(this.container, {
            videoId: this.videoId,
            // not using youtube's loop because it only work for playlists, instead it's manually restarting when done
            playerVars: {
                autoplay: this.autoplay ? 1 : 0,
                rel: 0,
                showinfo: 0,
                modestbranding: 1,
                cc_load_policy: this.captions ? 1 : 0,
                controls: this.hideControls ? 0 : 1,
                start: this.start,
                end: this.end,
            },
            events: {
                onStateChange: (e: YT.OnStateChangeEvent) => {
                    const stateName = this.stateName(e.data);

                    const eventName = 'youtube-' + stateName.toLowerCase();
                    this.dispatchEvent(new Event(eventName));
                },
                onReady: () => {
                    if(this.muted) {
                        this.player?.mute();
                    }

                    this.dispatchEvent(new Event("ready"));
                },
            }
        });
    }

    private stateName(num: number): StateName {
        const state = YT.PlayerState;
        return Object.keys(state).find(key => (state as any)[key] === num)?.toLowerCase() as StateName;
    }

    render() {
        return html`
            <div id="container"></div>
        `;
    }
}

class YouTubeEvents extends EventTarget {

    private _apiReady: boolean = false;
    get apiReady(): boolean {
        return this._apiReady;
    }

    constructor() {
        super();

        this.addCallback();
        this.createScript();
    }

    private createScript() {
        var tag = document.createElement('script');

        tag.src = "https://www.youtube.com/iframe_api";
        var firstScriptTag = document.getElementsByTagName('script')[0];
        firstScriptTag.parentNode!.insertBefore(tag, firstScriptTag);
    }

    private addCallback() {
        window.onYouTubeIframeAPIReady = () => {
            this._apiReady = true;
            this.dispatchEvent(new Event("api-ready"));
        }
    }
}
