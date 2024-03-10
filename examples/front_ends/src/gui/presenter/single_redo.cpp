// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "single_redo.h"
#include "event_dispatcher.h"
#include "undo_redo/undo_redo_interactor.h"

using namespace FrontEnds::Interactor;
using namespace FrontEnds::Interactor::UndoRedo;
using namespace FrontEnds::Presenter;

SingleRedo::SingleRedo(QObject *parent)
    : QObject{parent}
{
    m_action = UndoRedoInteractor::instance()->createUndoAction(this, tr("Redo: %1"));

    m_enabled = m_action->isEnabled();
    connect(m_action, &QAction::enabledChanged, this, [this](bool newEnabled) {
        if (m_enabled == newEnabled)
            return;
        m_enabled = newEnabled;
        Q_EMIT enabledChanged();
    });

    m_text = m_action->text();
    connect(m_action, &QAction::changed, this, [this]() {
        const QString &newText = m_action->text();
        if (m_text == newText)
            return;
        m_text = newText;
        Q_EMIT textChanged();
    });

    connect(EventDispatcher::instance()->undoRedo(), &UndoRedoSignals::redoing, this, [this](Scope scope, bool active) {
        if (m_enabled == active)
            return;
        m_enabled = active;
        Q_EMIT enabledChanged();
    });
}

bool SingleRedo::enabled() const
{
    return m_enabled;
}

QString SingleRedo::text() const
{
    return m_text;
}

void SingleRedo::redo()
{
    UndoRedoInteractor::instance()->redo();
}