// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "undo_redo_interactor.h"

#include <QCoroSignal>

using namespace FrontEnds::Interactor;
using namespace FrontEnds::Interactor::UndoRedo;
using namespace Qleany::Tools::UndoRedo;

QPointer<UndoRedoInteractor> UndoRedoInteractor::s_instance = nullptr;

UndoRedoInteractor::UndoRedoInteractor(ThreadedUndoRedoSystem *undo_redo_system, QSharedPointer<EventDispatcher> eventDispatcher)
    : QObject{nullptr}
{
    // connections for undo commands:
    m_undo_redo_system = undo_redo_system;
    m_eventDispatcher = eventDispatcher;

    auto *undoRedoSignals = m_eventDispatcher->undoRedo();

    connect(m_undo_redo_system, &ThreadedUndoRedoSystem::stateChanged, undoRedoSignals, &UndoRedoSignals::stateChanged);
    connect(m_undo_redo_system, &ThreadedUndoRedoSystem::redoing, undoRedoSignals, &UndoRedoSignals::redoing);
    connect(m_undo_redo_system, &ThreadedUndoRedoSystem::undoing, undoRedoSignals, &UndoRedoSignals::undoing);

    s_instance = this;
}

UndoRedoInteractor *UndoRedoInteractor::instance()
{
    return s_instance.data();
}

bool UndoRedoInteractor::canUndo() const
{
    return m_undo_redo_system->canUndo();
}

bool UndoRedoInteractor::canRedo() const
{
    return m_undo_redo_system->canRedo();
}

void UndoRedoInteractor::setUndoLimit(int limit)
{
    m_undo_redo_system->setUndoLimit(limit);
}

int UndoRedoInteractor::undoLimit() const
{
    return m_undo_redo_system->undoLimit();
}

QString UndoRedoInteractor::undoText() const
{
    return m_undo_redo_system->undoText();
}

QString UndoRedoInteractor::redoText() const
{
    return m_undo_redo_system->redoText();
}

QStringList UndoRedoInteractor::undoRedoTextList() const
{
    return m_undo_redo_system->undoRedoTextList();
}

int UndoRedoInteractor::currentIndex() const
{
    return m_undo_redo_system->currentIndex();
}

QUuid UndoRedoInteractor::activeStackId() const
{
    return m_undo_redo_system->activeStackId();
}

QStringList UndoRedoInteractor::queuedCommandTextListByScope(const QString &scopeFlagString) const
{
    return m_undo_redo_system->queuedCommandTextListByScope(scopeFlagString);
}

bool UndoRedoInteractor::isRunning() const
{
    return m_undo_redo_system->isRunning();
}

int UndoRedoInteractor::numberOfCommands() const
{
    return m_undo_redo_system->numberOfCommands();
}

QAction *UndoRedoInteractor::createUndoAction(QObject *parent, const QString &prefix) const
{
    return m_undo_redo_system->createUndoAction(parent, prefix);
}

QAction *UndoRedoInteractor::createRedoAction(QObject *parent, const QString &prefix) const
{
    return m_undo_redo_system->createRedoAction(parent, prefix);
}

void UndoRedoInteractor::undo()
{
    return m_undo_redo_system->undo();
}

void UndoRedoInteractor::redo()
{
    return m_undo_redo_system->redo();
}

void UndoRedoInteractor::clear()
{
    return m_undo_redo_system->clear();
}

void UndoRedoInteractor::setCurrentIndex(int index)
{
    return m_undo_redo_system->setCurrentIndex(index);
}

void UndoRedoInteractor::setActiveStack(const QUuid &stackId)
{
    return m_undo_redo_system->setActiveStack(stackId);
}