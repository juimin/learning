import React from 'react';
import './css/Landing.css'
import * as firebase from 'firebase';

import PlacesLoader from './PlacesLoader';

firebase.initializeApp({
    apiKey: "AIzaSyBhezhfpFjdawnH4B3OWpg2BEW_JDyr5dA",
    authDomain: "disasterelief-a11dd.firebaseapp.com",
    databaseURL: "https://disasterelief-a11dd.firebaseio.com",
    storageBucket: "disasterelief-a11dd.appspot.com",
    messagingSenderId: "383411681087"
});

var disasterBase = firebase.database().ref('disasters');

var LandingPage = React.createClass({

    getInitialState() {
        return {disasterList:[]};
    },

    componentDidMount(){
        var blank = [];
        return disasterBase.once('value')
            .then(snapshot => snapshot.val())
            .then(data => blank = data)
            .then(data => this.setState({disasterList:blank}))
            //.then(data => console.log("this is my array:", blank))
    },

    render() {
        return(
            <div className="landing">
                <h1 id="title">DREAM: Disaster Relief Emergency Assistance Mapping</h1>
                <div className="map" id="map">
                <PlacesLoader disasters={this.state.disasterList} />
                </div>
                <h5 id="authors">By Alex Gilbert, Davin Lee, and Derek Wang</h5>
            </div>
        )
    }
});

export default LandingPage;
