import { emit, listen } from '@tauri-apps/api/event';
import { invoke } from '@tauri-apps/api/tauri';

export type RendererInfo = {
    windowIndex: number;
    numDisplays: number;
    isSingleWindow: boolean;
}

export type Comment = {
    id: number;
    text: string;
    offsetTopRatio: number;
}

export class Ipc {
    static notifyCommentArrivedToLeftEdge(comment: Comment, windowIndex: number) {
        emit('comment-arrived-to-left-edge', { comment: comment, windowIndex: windowIndex });
    }

    static requestDuration(callback: (duration: number) => void) {
        invoke('request_duration').then((result) => callback(result as number));
    }

    static requestDefaultDuration(callback: (duration: number) => void) {
        invoke('request_default_duration').then((result) => callback(result as number));
    }

    static requestTextColorStyle(callback: (style: string) => void) {
        invoke('request_text_color_style').then((result) => callback(result as string));
    }

    static requestTextStrokeStyle(callback: (style: string) => void) {
        invoke('request_text_stroke_style').then((result) => callback(result as string));
    }

    static requestNewlineEnabled(callback: (isEnabled: boolean) => void) {
        invoke('request_newline_enabled').then((result) => callback(result as boolean));
    }

    static requestIconEnabled(callback: (isEnabled: boolean) => void) {
        invoke('request_icon_enabled').then((result) => callback(result as boolean));
    }

    static requestInlineImgEnabled(callback: (isEnabled: boolean) => void) {
        invoke('request_inline_img_enabled').then((result) => callback(result as boolean));
    }

    static requestImgEnabled(callback: (isEnabled: boolean) => void) {
        invoke('request_img_enabled').then((result) => callback(result as boolean));
    }

    static requestVideoEnabled(callback: (isEnabled: boolean) => void) {
        invoke('request_video_enabled').then((result) => callback(result as boolean));
    }

    static requestRoundIconEnabled(callback: (isEnabled: boolean) => void) {
        invoke('request_round_icon_enabled').then((result) => callback(result as boolean));
    }

    static onCommentReceived(callback: (comment: Comment, rendererInfo: RendererInfo) => void) {
        listen('comment', (event: any) => {
            const comment = event.payload.comment as Comment;
            const rendererInfo = event.payload.rendererInfo as RendererInfo;
            callback(comment, rendererInfo);
        })
    }

    static onTogglePause(callback: () => void) {
        listen('toggle-pause', () => callback());
    }

    static onDurationUpdated(callback: (duration: number) => void) {
        listen('update-duration', (event: any) => callback(event.payload.duration as number));
    }

    static onUpdateNewlineEnabled(callback: (isEnabled: boolean) => void) {
        listen('update-newline-enabled', (event: any) => callback(event.payload.isEnabled as boolean));
    }

    static onUpdateIconEnabled(callback: (isEnabled: boolean) => void) {
        listen('update-icon-enabled', (event: any) => callback(event.payload.isEnabled as boolean));
    }

    static onUpdateInlineImgEnabled(callback: (isEnabled: boolean) => void) {
        listen('update-inline-img-enabled', (event: any) => callback(event.payload.isEnabled as boolean));
    }

    static onUpdateImgEnabled(callback: (isEnabled: boolean) => void) {
        listen('update-img-enabled', (event: any) => callback(event.payload.isEnabled as boolean));
    }

    static onUpdateVideoEnabled(callback: (isEnabled: boolean) => void) {
        listen('update-video-enabled', (event: any) => callback(event.payload.isEnabled as boolean));
    }

    static onUpdateRoundIconEnabled(callback: (isEnabled: boolean) => void) {
        listen('update-round-icon-enabled', (event: any) => callback(event.payload.isEnabled as boolean));
    }
}
