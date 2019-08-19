import React from 'react';
import ReactCursorPosition from 'react-cursor-position';
import {debounce} from './lib';

const URL = 'ws://localhost:8080/ws/';
const WS = new WebSocket(URL);


export default class CursorTracker extends React.Component {
    render() {
        return (
            <ReactCursorPosition className="cursor-tracker">
                <PositionLabel />
            </ReactCursorPosition>
        );
    }
}

const PositionLabel = (props) => {

    const {
        detectedEnvironment: {
            isMouseDetected = false,
            isTouchDetected = false
        } = {},
        elementDimensions: {
            width = 0,
            height = 0
        } = {},
        isActive = false,
        isPositionOutside = false,
        position: {
            x = 0,
            y = 0
        } = {}
    } = props;

    const msg = ` \
        { "Point": { \
        "x": ${props.position.x / props.elementDimensions.width}, \
        "y": ${props.position.y / props.elementDimensions.height}}} \
        `
    if (WS.readyState === WS.OPEN) {
        debounce(() => {
            WS.send(msg);
        }, 5)();
    } else {
        console.log("WebSocket isn't open yet");
    }

    return (
        <div className="example__label">
            {`x: ${x / width}`}<br />
            {`y: ${y / height}`}<br />
            {`width:: ${width}`}<br />
            {`height: ${height}`}<br />
            {`isActive: ${isActive}`}<br />
            {`isPositionOutside: ${isPositionOutside ? 'true' : 'false'}`}<br />
            {`isMouseDetected: ${isMouseDetected ? 'true' : 'false'}`}<br />
            {`isTouchDetected: ${isTouchDetected ? 'true' : 'false'}`}
        </div>
    );
};
