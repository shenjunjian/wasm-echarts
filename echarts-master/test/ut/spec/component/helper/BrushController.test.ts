/*
* Licensed to the Apache Software Foundation (ASF) under one
* or more contributor license agreements.  See the NOTICE file
* distributed with this work for additional information
* regarding copyright ownership.  The ASF licenses this file
* to you under the Apache License, Version 2.0 (the
* "License"); you may not use this file except in compliance
* with the License.  You may obtain a copy of the License at
*
*   http://www.apache.org/licenses/LICENSE-2.0
*
* Unless required by applicable law or agreed to in writing,
* software distributed under the License is distributed on an
* "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
* KIND, either express or implied.  See the License for the
* specific language governing permissions and limitations
* under the License.
*/

import BrushController from '../../../../../src/component/helper/BrushController';
import { createChart } from '../../../core/utHelper';
import { EChartsType } from '../../../../../src/echarts';
import Rect from 'zrender/src/graphic/shape/Rect';


describe('component/helper/BrushController', function () {

    let chart: EChartsType;

    beforeEach(function () {
        chart = createChart();
    });

    afterEach(function () {
        chart.dispose();
    });

    it('updateCovers inherits the enabled brush style', function () {
        const controller = new BrushController(chart.getZr()).mount();
        const brushStyle = {
            fill: 'rgba(255, 0, 0, 0.35)',
            stroke: 'rgb(255, 0, 0)',
            lineWidth: 6,
            opacity: 0.45
        };

        controller
            .enableBrush({
                brushType: 'lineX',
                brushStyle: brushStyle,
                transformable: false,
                removeOnClick: true
            })
            .updateCovers([{
                brushType: 'lineX',
                range: [20, 60]
            }]);

        // @ts-ignore access internal state for behavior verification.
        const cover = controller._covers[0];
        // @ts-ignore access internal state for behavior verification.
        const brushOption = cover.__brushOption;
        const mainEl = cover.childAt(0) as Rect;

        expect(brushOption.brushStyle).toEqual(brushStyle);
        expect(brushOption.transformable).toEqual(false);
        expect(brushOption.removeOnClick).toEqual(true);
        expect(mainEl.style.fill).toEqual(brushStyle.fill);
        expect(mainEl.style.stroke).toEqual(brushStyle.stroke);
        expect(mainEl.style.lineWidth).toEqual(brushStyle.lineWidth);
        expect(mainEl.style.opacity).toEqual(brushStyle.opacity);
    });

});
