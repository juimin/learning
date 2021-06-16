import React from 'react';
import ReactDOM from 'react-dom';
import App from './App';
import { IndexRoute, Router, Route, Link, hashHistory } from 'react-router';
import './css/index.css';
import LandingPage from './LandingPage';
import Template from './Template';
import Info from './Info';
// Render DOM -- wrapper in MuiThemeProvider for material-ui elements

ReactDOM.render(
        <Router history={hashHistory}>
            <Route path="/" component={App}>
                <IndexRoute component={LandingPage}/>
                <Route path="Template" component={Template}/>
                <Route path="Info" component={Info}/>
                
            </Route>
        </Router>,
  document.getElementById('root')
);
