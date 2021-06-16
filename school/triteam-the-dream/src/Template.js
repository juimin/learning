import React from 'react';
import $ from 'jquery';
import * as firebase from 'firebase';
import Router from 'react-router';
import './css/Template.css'

var disasterBase = firebase.database().ref('disasters');

function getLocation() {
		if (navigator.geolocation){
			navigator.geolocation.getCurrentPosition(fillPosition);
		} else {
			alert("Geolocation not supported!");
		}
	}

	function fillPosition(position) {
		$("#lat").val(position.coords.latitude);
		$("#long").val(position.coords.longitude);
	}

	function clearLocation() {
		$("#lat").val('');
		$("#long").val('');
	}

	function addDis() {
		event.preventDefault();
		console.log("adding to firebase");
		disasterBase.push({
                name: $('#name').val(),
                description: $('#description').val(),
                lat: Number($('#lat').val()).toFixed(3),  //$('#lat').val(),
                long: Number($('#long').val()).toFixed(3),  //$('#long').val(),

            });
		$('#name').val('');
		$('#description').val('');
		$('#lat').val('');
		$('#long').val('');
		window.location.assign('./');
	}


//on submit of the form in Template, pass all this data to the ace space that is the firebase place
var Template = React.createClass({

	render() {
		return(
			<div id="every">
			<div id="enter">
				<h1>Report an Event</h1>
				<input type="button" className="butt" value="Get My Current Location" name="getLoc" onClick={getLocation}/><br/>
				<input type="button" className="butt" value="Clear Location" name="clearLoc" onClick={clearLocation}/><br/>
				<form id="eventForm" onSubmit={addDis}>
					<input type="text" className="in" placeholder="latitude" id="lat" required />
					<input type="text" className="in" placeholder="longitude" id="long" required /><br/>
					<input type="text" className="in" placeholder="name of location" id="name" required /> <br />
					<textarea placeholder="description of event" className="in" id="description" required /><br />
					<input type="submit" className="butt" value="Create an Event" />
				</form>
			</div>
			</div>
		)
	}
});

export default Template;
