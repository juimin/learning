package com.example.lazydevs.lazydevsfinalproject;

import android.support.v7.app.AppCompatActivity;
import android.os.Bundle;
import android.util.Log;

public class RecipeFragmentFrame extends AppCompatActivity {

    private final String TAG = "Recipe Frag Frame";

    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_recipe_fragment_frame);

        // Check that the activity is using the layout version with
        // the fragment_container FrameLayout
        if (findViewById(R.id.recipe_fragment_frame) != null) {
            // However, if we're being restored from a previous state,
            // then we don't need to do anything and should return or else
            // we could end up with overlapping fragments.
            if (savedInstanceState != null) {
                return;
            }

            int position = AppState.getInstance().getPosition();
            int recipeId = AppState.getInstance().getDishes().get(position).getID();
            RecipeQuery recipe = new RecipeQuery(recipeId, getSupportFragmentManager(), getIntent(), R.id.recipe_fragment_frame);
            recipe.setUrl();
            recipe.execute();
        }
    }

    @Override
    protected void onDestroy() {
        super.onDestroy();
        Log.e(TAG, "The Recipe Activity has closed");
    }

    @Override
    protected void onRestart() {
        super.onRestart();
        Log.i(TAG, "The Recipe Activity has restarted");
    }

    @Override
    protected void onPause() {
        super.onPause();
        Log.i(TAG, "The Recipe Activity has paused");
    }

    @Override
    protected void onStart() {
        super.onStart();
        Log.i(TAG, "The Recipe Activity has started");
    }

    @Override
    protected void onResume() {
        super.onResume();
        Log.i(TAG, "The Recipe Activity has resumed");
    }

    @Override
    protected void onStop() {
        super.onStop();
        Log.e(TAG, "The Recipe Activityhas stopped");
    }
}
