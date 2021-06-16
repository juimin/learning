import React from 'react';
import $ from 'jquery';
import Router from 'react-router';
import './css/Info.css'

var Info = React.createClass({
	render() {
		return(
			<div id="how">
				<h3> How It Works: </h3>
				<p>
					DREAM utilizes the power of people and technology to keep people safe.
					By allowing anyone to report disasters or other events and tracking crowdsourced data on disaster locations, it provides useful information that can be used in a multitude of ways, 
					from helping people making informed decisions about where and when to safely travel to aiding in disaster rescue efforts.
				</p>
				<p> 
					Report a disaster or other event via the "Report an Event" page, and it will be visible to the public on the map!
					Double click a marker on the map to remove it.
					
				</p>
				<br />
				<br />
				<p>
					Made with love for INFO 343 C in Autumn 2016 by Alex Gilbert, Davin Lee, and Derek Wang.
					Look forward to future improvements to this hopefully useful tool!

				</p>
			</div>
		)
	}
});

export default Info;