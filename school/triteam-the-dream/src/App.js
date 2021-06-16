import React from 'react';
import './css/App.css';
import { Link } from 'react-router';
import '../node_modules/font-awesome/css/font-awesome.css';

var App = React.createClass({
	render() {
		return (
				<div className="App">
					<div className="navbar">
						<Link className="link" activeClassName='active' to="/">Map</Link>
						<Link className="link" activeClassName='active' to="/Template">Report an Event</Link>
						<Link className="link" activeClassName='active' to="/Info" id="infoicon"><i className="fa fa-info-circle "/></Link>
						
					</div>
					<div className="children">
						{this.props.children}
					</div>
				</div>
		);
	}
});

export default App;
